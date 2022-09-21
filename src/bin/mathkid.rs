use mathkid::addition::Addition;
use mathkid::{Outcome, Profile, Question, Topic};
use std::{io, process};
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Error, stdout, Write};
use std::path::{PathBuf};
use tinyrand_std::thread_rand;

const PROFILE_DIR: &str = ".mathkid";
const USER: &str = "Anna";

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1)
    });
}

enum CliError {
    Io(io::Error),
    Other(String)
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(err) => err.fmt(f),
            CliError::Other(err) => err.fmt(f)
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: Error) -> Self {
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
    ensure_init_profile()?;
    run_course()
}

/// The path to the profile directory (i.e., `~/${PROFILE_DIR}`). Returns `None` if the home directory could not be established.
fn home_profile_dir() -> Result<PathBuf, CliError> {
    let home_dir = home::home_dir().ok_or("could not determine home directory")?;
    Ok(home_dir.join(PROFILE_DIR))
}

/// Obtains a list of existing (sanitised) profile names.
fn list_profiles() -> Result<Vec<String>, CliError> {
    let home_profile_dir = home_profile_dir()?;
    create_dir_all(home_profile_dir.clone())?;
    let contents = home_profile_dir.read_dir()?;
    let contents = contents.into_iter().collect::<Vec<_>>();
    contents
        .iter()
        .find(|entry| entry.is_err())
        .map_or(Ok(()), | entry| Err(entry.as_ref().err().unwrap().to_string()))?;

    let entries = contents
        .into_iter()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    Ok(entries)
}

/// Ensures that at least one user profile has been set up.
fn ensure_init_profile() -> Result<(), CliError> {
    let profiles = list_profiles()?;
    if profiles.is_empty() {
        println!("It appears we haven't met before. Let's set up a profile first.");
        print!("Your child's first name: ");
        stdout().flush()?;
        let first_name = readln(|str|!str.trim().is_empty()).ok_or("cannot continue without a name")?;
        let profile = Profile { first_name };
        let filename = format!("{}.profile.json", profile.sanitised_first_name());
        let out_file = File::create(home_profile_dir()?.join(filename))?;
        let mut writer = BufWriter::new(out_file);
        writer.write_all(profile.to_json()?.as_bytes())?;
        writer.flush()?;
    }
    Ok(())
}

fn run_course() -> Result<(), CliError> {
    println!("Hi {}, I've got a few questions for you.", USER);

    let mut rand = thread_rand();
    let topic = Addition::new();
    for question_no in 1..=10 {
        let question = topic.ask(&mut rand);
        ask_question(question_no, question);
    }

    println!("Congratulations, you've answered all my questions!");
    Ok(())
}

/// Asks the given question and keeps prompting the user until they either get it right or the
/// input with aborted (i.e., with a CTRL+D).
fn ask_question(question_no: u32, question: impl Question) {
    println!("Question {question_no}:");
    println!("{question}");
    loop {
        let answer = readln(|s| !s.trim().is_empty());
        match answer {
            None => {
                println!("You've skipped the question.");
                return;
            },
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
