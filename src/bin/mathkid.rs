use clap::Parser;
use itertools::Itertools;
use mathkid::syllabus::{Syllabus};
use mathkid::{syllabus, Outcome, Profile, Question, Topic};
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, File};
use std::io::{stdout, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::{io, process};
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

const PROFILE_DIR: &str = ".mathkid";
const DEF_QUESTIONS: u16 = 10;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1)
    });
}

enum CliError {
    Io(io::Error),
    Other(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(err) => err.fmt(f),
            CliError::Other(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> Self {
        CliError::Other(err)
    }
}

impl From<&str> for CliError {
    fn from(err: &str) -> Self {
        CliError::Other(err.to_string())
    }
}

#[derive(Debug, Clone)]
enum Listing {
    Topics,
    Courses,
    Profiles,
}

impl FromStr for Listing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "topics" => Ok(Listing::Topics),
            "courses" => Ok(Listing::Courses),
            "profiles" => Ok(Listing::Profiles),
            _ => Err(format!("unknown listing of type '{s}'")),
        }
    }
}

/// A maths tutor for kids.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Topic
    #[clap(short, long, value_parser)]
    topic: Option<String>,

    /// List {topics, courses, profiles}
    #[clap(short, long, value_parser)]
    list: Option<Listing>,

    /// Course
    #[clap(short, long, value_parser)]
    course: Option<String>,

    /// Number of questions per module
    #[clap(short, long, value_parser)]
    questions: Option<u16>,

    /// The profile to use
    #[clap(short, long, value_parser)]
    profile: Option<String>,
}

fn run() -> Result<(), CliError> {
    let syllabus = syllabus::presets::primary();
    let args = Args::parse();
    if let Some(listing) = args.list {
        match listing {
            Listing::Profiles => print_profiles()?,
            Listing::Courses => print_courses(&syllabus),
            Listing::Topics => print_topics(&syllabus, &args.course)?,
        }
        return Ok(());
    }

    ensure_init_profile(&syllabus)?;

    let profile_name = match args.profile {
        None => {
            let profiles = get_profile_names()?;
            if profiles.len() != 1 {
                return Err(CliError::Other(format!(
                    "please select a profile (try --list profiles)"
                )));
            }
            profiles[0].clone()
        }
        Some(profile_name) => {
            if !get_profile_names()?.contains(&profile_name) {
                return Err(CliError::Other(format!("no such profile '{profile_name}'")));
            }
            profile_name
        }
    };

    let profile = load_profile(&profile_name)?;
    let syllabus = syllabus::presets::primary();
    let course_name = match args.course {
        None => profile.course,
        Some(course_name) => course_name,
    };

    let course = syllabus
        .courses
        .get(&course_name)
        .ok_or(CliError::Other(format!("no such course '{course_name}' (try --list courses)")))?;

    let topics: Vec<&Box<dyn Topic>> = match args.topic {
        None => course.modules.values().collect(),
        Some(topic_name) => {
            let topics = course
                .modules
                .values()
                .filter(|topic| topic.name() == topic_name)
                .collect::<Vec<_>>();
            if topics.is_empty() {
                return Err(CliError::Other(format!(
                    "no such topic '{topic_name}' in course '{course_name}' (try --list topics)"
                )));
            }
            topics
        }
    };

    let questions: Option<u16> = args.questions;
    run_topics(
        topics,
        questions.unwrap_or(DEF_QUESTIONS),
        &profile.first_name,
    )?;
    Ok(())
}

/// The path to the profile directory (i.e., `~/${PROFILE_DIR}`). Returns `None` if the home directory could not be established.
fn home_profile_dir() -> Result<PathBuf, CliError> {
    let home_dir = home::home_dir().ok_or("could not determine home directory")?;
    Ok(home_dir.join(PROFILE_DIR))
}

/// Obtains a list of existing (sanitised) profile names.
fn get_profile_names() -> Result<Vec<String>, CliError> {
    let home_profile_dir = home_profile_dir()?;
    if !home_profile_dir.is_dir() {
        return Ok(vec![]);
    }

    let contents = home_profile_dir.read_dir()?;
    let contents = contents.into_iter().collect::<Vec<_>>();
    contents
        .iter()
        .find(|entry| entry.is_err())
        .map_or(Ok(()), |entry| {
            Err(entry.as_ref().err().unwrap().to_string())
        })?;

    let entries = contents
        .into_iter()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .filter(|entry| entry.ends_with(".profile.json"))
        .map(|entry| {
            let index = entry.find(".").unwrap();
            String::from_utf8_lossy(&entry.as_bytes()[0..index]).to_string()
        })
        .sorted()
        .collect::<Vec<_>>();
    Ok(entries)
}

fn print_profiles() -> Result<(), CliError> {
    let profiles = get_profile_names()?;
    println!("The following profiles are available:");
    for profile in profiles {
        println!("  {profile}");
    }
    Ok(())
}

fn print_courses(syllabus: &Syllabus) {
    println!("The following courses are available:");
    for course in syllabus.courses.keys().sorted() {
        println!("  {course}");
    }
}

fn print_topics(syllabus: &Syllabus, course: &Option<String>) -> Result<(), String> {
    println!("The following topics are available:");
    let topics = match course {
        None => syllabus.get_topic_names(),
        Some(course) => {
            let course = syllabus
                .courses
                .get(course)
                .ok_or(format!("no such course '{course}'"))?;
            course.get_topic_names()
        }
    };
    for topic in topics {
        println!("  {topic}");
    }
    Ok(())
}

/// Ensures that at least one user profile has been set up.
fn ensure_init_profile(syllabus: &Syllabus) -> Result<(), CliError> {
    let profiles = get_profile_names()?;
    if profiles.is_empty() {
        println!("It appears we haven't met before. Let's set up a profile first.");
        print!("Your child's first name: ");
        stdout().flush()?;
        let first_name =
            readln(|str| !str.trim().is_empty()).ok_or("cannot continue without a name")?;

        println!("We need to enroll {first_name} into a course.");
        print_courses(syllabus);
        let courses = syllabus.courses.keys().collect::<Vec<_>>();
        print!("Course: ");
        stdout().flush()?;
        let course = readln(|str| courses.contains(&&str.trim().to_string()))
            .ok_or("cannot continue without a course")?;

        let profile = Profile { first_name, course };
        write_profile(&profile)?;
    }
    Ok(())
}

/// Writes the given profile to the file system.
fn write_profile(profile: &Profile) -> Result<(), CliError> {
    let filename = format!("{}.profile.json", profile.sanitised_first_name());
    let home_profile_dir = home_profile_dir()?;
    create_dir_all(home_profile_dir.clone())?;
    let out_file = File::create(home_profile_dir.join(filename))?;
    let mut writer = BufWriter::new(out_file);
    writer.write_all(profile.to_json()?.as_bytes())?;
    writer.flush()?;
    Ok(())
}

/// Loads a profile from the file system, given its name.
fn load_profile(profile_name: &String) -> Result<Profile, CliError> {
    let home_profile_dir = home_profile_dir()?;
    let filename = format!("{}.profile.json", profile_name);
    let in_file = File::open(home_profile_dir.join(filename))?;
    let mut reader = BufReader::new(in_file);
    let mut json = String::new();
    reader.read_to_string(&mut json)?;
    let profile = Profile::from_json(&json)?;
    Ok(profile)
}

/// Ask questions from a list of topics.
fn run_topics(
    topics: Vec<&Box<dyn Topic>>,
    questions: u16,
    first_name: &str,
) -> Result<(), CliError> {
    println!("Hi {}, I've got a few questions for you.", first_name);

    let mut rand: Box<dyn RandRange<u32>> = Box::new(thread_rand());
    for topic in topics {
        println!("Topic: {}", topic.name());
        for question_no in 1..=questions {
            let question = topic.ask(&mut rand);
            ask_question(question_no, question);
        }
    }

    println!("Congratulations, you've answered all my questions!");
    Ok(())
}

/// Asks the given question and keeps prompting the user until they either get it right or the
/// input with aborted (i.e., with a CTRL+D).
fn ask_question(question_no: u16, question: Box<dyn Question>) {
    println!("Question {question_no}:");
    println!("{question}");
    loop {
        let answer = readln(|s| !s.trim().is_empty());
        match answer {
            None => {
                println!("You've skipped the question.");
                return;
            }
            Some(answer) => {
                let answer = answer;
                match question.answer(&answer) {
                    Outcome::Incorrect => {
                        println!("Your answer isn't quite right. Try again!")
                    }
                    Outcome::Invalid(err) => {
                        println!("There was a problem with your answer: {err}")
                    }
                    Outcome::Correct => {
                        println!("That's the right answer. Great work!");
                        return;
                    }
                }
            }
        }
    }
}

/// Reads from `stdin`, trimming whitespace from the input. The trimmed input is subjected
/// to a predicate test. If the predicate passes (returns `true`), the input is returned; otherwise,
/// the user is prompted again.
///
/// Returns `None` if no input was read (i.e., the read was aborted with a CTRL+D).
fn readln(mut predicate: impl FnMut(&str) -> bool) -> Option<String> {
    loop {
        let mut buf = String::new();
        let bytes = io::stdin().read_line(&mut buf).unwrap();
        match bytes {
            0 => return None,
            _ => {
                let buf = buf.trim();
                if predicate(buf) {
                    return Some(buf.to_string());
                } else {
                    println!("I don't know what you mean.")
                }
            }
        }
    }
}
