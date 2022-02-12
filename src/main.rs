use regex::{self, Regex};
use std::{
    io::{self, Write},
    iter::repeat,
    str::FromStr,
};

enum MetaCommand {
    Exit,
}

impl MetaCommand {
    fn is_meta_command(input_str: &str) -> bool {
        input_str.starts_with('.')
    }
}

enum MetaCommandParseError {
    UnrecognizedCommand,
}

impl FromStr for MetaCommand {
    type Err = MetaCommandParseError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        match input_str {
            ".exit" => Ok(MetaCommand::Exit),
            _ => Err(MetaCommandParseError::UnrecognizedCommand),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Statement {
    Insert,
    Select,
}
enum StatementParseError {
    UnrecognizedStatement,
}

impl FromStr for Statement {
    type Err = StatementParseError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        const VALUES: [Statement; 2] = [Statement::Insert, Statement::Select];
        const NAMES: [&str; 2] = ["insert", "select"];
        NAMES
            .iter()
            .position(|&name| input_str.starts_with(name))
            .ok_or(StatementParseError::UnrecognizedStatement)
            .map(|index| VALUES[index])
    }
}

#[derive(Debug)]
enum ParseError {
    RegexCreation(regex::Error),
    NoRegexMatch,
}

impl From<regex::Error> for ParseError {
    fn from(err: regex::Error) -> ParseError {
        ParseError::RegexCreation(err)
    }
}

#[derive(Debug)]
struct Schema {
    id: u8,
    username: [char; 32],
    email: [char; 32],
}

fn to_fixlen_array(input: &str) -> [char; 32] {
    let vec: Vec<char> = input.chars().chain(repeat('\0')).take(32).collect();
    vec.try_into().unwrap()
}

fn parse_insert(input_str: &str) -> Result<Schema, ParseError> {
    let re = Regex::new(r"insert (?P<id>\d+) (?P<username>\w+) (?P<email>\w+@\w+\.\w+)")?;
    let caps = re.captures(input_str);
    match caps {
        Some(captures) => Ok(Schema {
            id: std::str::FromStr::from_str(&captures["id"]).unwrap(),
            username: to_fixlen_array(&captures["username"]),
            email: to_fixlen_array(&captures["email"]),
        }),
        None => Err(ParseError::NoRegexMatch),
    }
}

#[derive(Debug)]
struct Table {
    rows: Vec<Schema>,
}

fn main() -> std::io::Result<()> {
    let mut user_input = String::new();
    let mut table = Table { rows: Vec::new() };
    loop {
        print!("db>");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();
        if MetaCommand::is_meta_command(&user_input) {
            match user_input.parse::<MetaCommand>() {
                Ok(MetaCommand::Exit) => return Ok(()),
                Err(_) => println!("unrecognized meta command: {}", &user_input),
            }
        } else if let Ok(stmt) = user_input.parse::<Statement>() {
            if stmt == Statement::Insert {
                match parse_insert(&user_input) {
                    Ok(row) => table.rows.push(row),
                    Err(e) => println!("Encountered error when parsing insert: {:?}", e),
                }
            } else if stmt == Statement::Select {
                println!("{:?}", table)
            }
        } else {
            println!("Unrecognized input: {}", &user_input);
        }
        user_input.clear();
    }
}
