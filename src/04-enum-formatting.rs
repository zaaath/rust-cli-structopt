use structopt::StructOpt;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcli", about = "Rust CLI examples with StructOpt")]
struct Opt {
    #[structopt(help = "Status of the task (todo, inprogress, blocked, done)")]
    status: Status
}

#[derive(Debug)]
enum Status {
    ToDo,
    InProgress,
    Done,
    Blocked
}

impl FromStr for Status {
    // TODO: understand why I need this line
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: understand how to convert to lowercase here without String::from later
        match s {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "blocked" => Ok(Status::Blocked),
            "done" => Ok(Status::Done),
            // TODO: understand how to simplify this
            _ => Err(ParseError(String::from("Status must be one of the following: todo, inprogress, blocked, or done.")))
        }
    }
}

// TODO: research ways to reuse well-known errors
#[derive(Debug)]
struct ParseError(String);
// TODO: understand the difference between std::error::Error and std::fmt::Error
impl std::error::Error for ParseError { }
impl Display for ParseError {
    // TODO: understand why lifetime specifier is required here
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
}
