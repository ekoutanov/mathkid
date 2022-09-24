use crate::args::{Args, Listing};
use crate::persistence::{get_profile_names, load_profile, write_profile};
use crate::print::{courses, horizontal_line, profiles, topics};
use mathkid::syllabus::Syllabus;
use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::{io, process};
use tinyrand_std::thread_rand;
use mathkid::profile::Profile;
use mathkid::syllabus;
use mathkid::topic::{Outcome, Question, Topic};

const DEF_QUESTIONS: u16 = 10;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1)
    });
}

pub enum CliError {
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

fn run() -> Result<(), CliError> {
    let syllabus = syllabus::presets::primary()?;
    let args = Args::parse_args();
    if let Some(listing) = args.list {
        match listing {
            Listing::Profiles => profiles(get_profile_names()?),
            Listing::Courses => courses(&syllabus),
            Listing::Topics => topics(&syllabus, &args.course)?,
        }
        return Ok(());
    }

    ensure_init_profile(&syllabus)?;

    let profile_name = match args.profile {
        None => {
            let profiles = get_profile_names()?;
            if profiles.len() != 1 {
                return Err(CliError::Other(String::from(
                    "please select a profile (try --list profiles)",
                )));
            }
            profiles[0].clone()
        }
        Some(profile_name) => {
            if !get_profile_names()?.contains(&profile_name) {
                return Err(CliError::Other(format!(
                    "no such profile '{profile_name}' (try --list profiles)"
                )));
            }
            profile_name
        }
    };

    let (profile, path) = load_profile(&profile_name)?;
    println!("Loaded profile from '{}'.", path.to_str().unwrap());
    println!();
    let syllabus = syllabus::presets::primary()?;
    let course_name = match args.course {
        None => profile.course,
        Some(course_name) => course_name,
    };

    let course = syllabus.courses.get(&course_name).ok_or_else(|| {
        CliError::Other(format!(
            "no such course '{course_name}' (try --list courses)"
        ))
    })?;

    let topics = match args.topic {
        None => course.modules.values().map(|topic| &**topic).collect(),
        Some(topic_name) => {
            let topics = course
                .modules
                .values()
                .map(|topic| &**topic)
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
        courses(syllabus);
        let courses = syllabus.courses.keys().collect::<Vec<_>>();
        print!("Course: ");
        stdout().flush()?;
        let course = readln(|str| courses.contains(&&str.trim().to_string()))
            .ok_or("cannot continue without a course")?;

        let profile = Profile { first_name, course };
        let out_file = write_profile(&profile)?;

        println!(
            "{}'s profile has been saved to '{}'.",
            profile.first_name,
            out_file.to_str().unwrap()
        );
        println!("You can edit it later.");
        println!();
    }
    Ok(())
}

/// Ask questions from a list of topics.
fn run_topics(topics: Vec<&dyn Topic>, questions: u16, first_name: &str) -> Result<(), CliError> {
    const YELLOW: &str = ansi::YELLOW;
    const RESET: &str = ansi::RESET;
    println!("Hi {}, I've got a few questions for you.", first_name);

    let mut rand = thread_rand();
    for topic in topics {
        println!("Topic: {YELLOW}{}{RESET}", topic.name());
        for question_no in 1..=questions {
            let question = topic.ask(&mut rand);
            ask_question(question_no, question.as_ref())?;
        }
    }

    println!("Congratulations, you've answered all my questions!");
    println!("Bye!");
    Ok(())
}

/// Asks the given question and keeps prompting the user until they either get it right or
/// the input is aborted (i.e., with a CTRL+D).
fn ask_question(question_no: u16, question: &dyn Question) -> Result<(), io::Error> {
    const CYAN: &str = ansi::CYAN;
    const RESET: &str = ansi::RESET;
    horizontal_line();
    println!("Question {question_no}:");
    println!("{CYAN}{question}{RESET}");
    loop {
        print!("Your answer: ");
        stdout().flush()?;
        let answer = readln(|s| !s.trim().is_empty());
        match answer {
            None => {
                println!("You've skipped the question.");
                return Ok(());
            }
            Some(answer) => {
                let answer = answer;
                match question.answer(&answer) {
                    Outcome::Incorrect => {
                        println!("Your answer isn't quite right. Try again!");
                    }
                    Outcome::Invalid(err) => {
                        println!("There was a problem with your answer: {err}");
                    }
                    Outcome::Correct => {
                        println!("That's the right answer. Great work!");
                        return Ok(());
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
                }
                println!("I don't know what you mean.");
            }
        }
    }
}

/// Argument parsing.
mod args {
    use clap::Parser;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub enum Listing {
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

    /// Maths questions for kids.
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Topic
        #[clap(short, long, value_parser)]
        pub topic: Option<String>,

        /// List {topics, courses, profiles}
        #[clap(short, long, value_parser)]
        pub list: Option<Listing>,

        /// Course
        #[clap(short, long, value_parser)]
        pub course: Option<String>,

        /// Number of questions per module
        #[clap(short, long, value_parser)]
        pub questions: Option<u16>,

        /// The profile to use
        #[clap(short, long, value_parser)]
        pub profile: Option<String>,
    }

    impl Args {
        pub fn parse_args() -> Args {
            Args::parse()
        }
    }
}

/// File I/O operations.
mod persistence {
    use crate::CliError;
    use itertools::Itertools;
    use std::fs::{create_dir_all, File};
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::path::PathBuf;
    use mathkid::profile::Profile;

    const PROFILE_DIR: &str = ".mathkid";

    /// The path to the profile directory (i.e., `~/${PROFILE_DIR}`). Returns `None` if the home directory could not be established.
    pub fn home_profile_dir() -> Result<PathBuf, CliError> {
        let home_dir = home::home_dir().ok_or("could not determine home directory")?;
        Ok(home_dir.join(PROFILE_DIR))
    }

    /// Obtains a list of existing (sanitised) profile names.
    pub fn get_profile_names() -> Result<Vec<String>, CliError> {
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
            .map(Result::unwrap)
            .map(|entry| entry.file_name().to_str().unwrap().to_string())
            .filter(|entry| entry.ends_with(".profile.json"))
            .map(|entry| {
                let index = entry.find('.').unwrap();
                String::from_utf8_lossy(&entry.as_bytes()[0..index]).to_string()
            })
            .sorted()
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Writes the given profile to the file system.
    pub fn write_profile(profile: &Profile) -> Result<PathBuf, CliError> {
        let filename = format!("{}.profile.json", profile.sanitised_first_name());
        let home_profile_dir = home_profile_dir()?;
        create_dir_all(home_profile_dir.clone())?;
        let out_path = home_profile_dir.join(filename);
        let out_file = File::create(out_path.clone())?;
        let mut writer = BufWriter::new(out_file);
        writer.write_all(profile.to_json()?.as_bytes())?;
        writer.flush()?;
        Ok(out_path)
    }

    /// Loads a profile from the file system, given its name.
    pub fn load_profile(profile_name: &String) -> Result<(Profile, PathBuf), CliError> {
        let home_profile_dir = home_profile_dir()?;
        let filename = format!("{}.profile.json", profile_name);
        let in_path = home_profile_dir.join(filename);
        let in_file = File::open(in_path.clone())?;
        let mut reader = BufReader::new(in_file);
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        let profile = Profile::from_json(&json)?;
        Ok((profile, in_path))
    }
}

/// Printing of output.
mod print {
    use crate::ansi;
    use itertools::Itertools;
    use mathkid::syllabus::Syllabus;

    /// Prints a horizontal line.
    pub fn horizontal_line() {
        println!("────────────────────────────────────────────────────────────────");
    }

    /// Prints the list of available profile names.
    pub fn profiles(profiles: Vec<String>) {
        const YELLOW: &str = ansi::YELLOW;
        const RESET: &str = ansi::RESET;
        println!("The following profiles are available:");
        for profile in profiles {
            println!("    {YELLOW}{profile}{RESET}");
        }
    }

    /// Prints the list of available courses in the syllabus.
    pub fn courses(syllabus: &Syllabus) {
        const YELLOW: &str = ansi::YELLOW;
        const RESET: &str = ansi::RESET;
        println!("The following courses are available:");
        for course in syllabus.courses.keys().sorted() {
            println!("    {YELLOW}{course}{RESET}");
        }
    }

    /// Prints the list of available topics in the syllabus. If `course` is supplied, the list of topics
    /// is reduced to those that are in the course.
    pub fn topics(syllabus: &Syllabus, course: &Option<String>) -> Result<(), String> {
        const YELLOW: &str = ansi::YELLOW;
        const RESET: &str = ansi::RESET;
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
            println!("    {YELLOW}{topic}{RESET}");
        }
        Ok(())
    }
}

pub mod ansi {
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[371m";
    pub const RESET: &str = "\x1b[0m";
}
