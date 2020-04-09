use structopt::StructOpt;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
enum Opt {
    Create {
        #[structopt(short, long)]
        debug: bool,

        #[structopt(short = "v", long = "velocity", default_value = "42")]
        speed: f64,
    },
    Edit {
        #[structopt(short, long)]
        profile: bool,

        #[structopt(long)]
        status: Option<Status>,
    },
}

#[derive(Debug)]
enum Status {
    ToDo,
    InProgress,
    Done,
    Blocked
}

impl FromStr for Status {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("Status::from_str {}", s);
        match s {
            "ToDo" => Ok(Status::ToDo),
            "InProgress" => Ok(Status::InProgress),
            "Done" => Ok(Status::Done),
            "Blocked" => Ok(Status::Blocked),
            _ => Err(MyError(String::from("Test MyError")))
        }
    }
}

#[derive(Debug)]
struct MyError(String);

impl std::error::Error for MyError { }

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Status must be one of the following: ToDo, InProgress, Done, or Blocked.")
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("main-1: opt = {:?}", opt);
    match opt {
        Opt::Create { debug, speed } => println!("Processing Opt::Create with debug = {}, speed = {}", debug, speed),
        Opt::Edit { profile, status } => println!("Processing Opt::Edit with profile = {}, status = {:?}", profile, status)
    }
}
