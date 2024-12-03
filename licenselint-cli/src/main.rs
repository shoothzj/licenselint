use chrono::{Datelike, Local};
use clap::{Arg, Command};
use licenselint::config::Config;
use licenselint::license::License;
use licenselint::linter::Linter;
use std::path::Path;

fn check(current_dir: &Path, linter: &Linter) {
    match linter.check_files_in_dir(current_dir) {
        Ok(issues) => {
            if issues.is_empty() {
                println!("No issues found.");
            } else {
                for issue in issues {
                    println!("Issue found in '{}'", issue.filename,);
                }
                std::process::exit(2);
            }
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Error checking files: {:?}", e);
            }
        }
    }
}

fn format(current_dir: &Path, linter: &Linter) {
    match linter.format_files_in_dir(current_dir) {
        Ok(_) => println!("Files formatted successfully."),
        Err(errors) => {
            for e in errors {
                eprintln!("Error formatting files: {:?}", e);
            }
        }
    }
}

fn main() {
    let matches = Command::new("licenselint-cli")
        .version("0.0.3")
        .about("A command-line tool for linting and fixing license formatting issues")
        .arg(
            Arg::new("author")
                .short('a')
                .long("author")
                .value_parser(clap::builder::ValueParser::string())
                .help("The author name to include in the license"),
        )
        .arg(
            Arg::new("email")
                .short('e')
                .long("email")
                .value_parser(clap::builder::ValueParser::string())
                .help("The author email to include in the license"),
        )
        .subcommand(Command::new("check").about("Check files for lint issues"))
        .subcommand(Command::new("format").about("Automatically format files to fix lint issues"))
        .get_matches();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let formatted_author = matches
        .get_one::<String>("author")
        .map(|s| s.as_str())
        .unwrap_or("Unknown Author")
        .to_string();

    let formatted_email = matches.get_one::<String>("email").map(|s| s.as_str());

    let formatted_author = if let Some(email) = formatted_email {
        format!("{} <{}>", formatted_author, email)
    } else {
        formatted_author
    };

    let current_year = Local::now().year().to_string();

    let config = Config::new_from_author(
        License::Apache20,
        formatted_author.to_string(),
        current_year,
    );

    let linter = Linter::new(&config);

    if matches.subcommand().is_none() {
        println!("No subcommand provided, defaulting to 'check'...");
        check(&current_dir, &linter);
    } else if matches.subcommand_matches("check").is_some() {
        check(&current_dir, &linter);
    } else if matches.subcommand_matches("format").is_some() {
        format(&current_dir, &linter);
    }
}
