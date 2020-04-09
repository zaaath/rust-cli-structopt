use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcli", about = "Rust CLI examples with StructOpt")]
enum Opt {
    #[structopt(about = "Create a new entity")]
    Create {
        title: String,
        text: Option<String>
    },
    #[structopt(about = "Edit an existing entity")]
    Edit {
        id: i32,
        title: Option<String>,
        text: Option<String>
    },
    #[structopt(about = "List all entities")]
    List
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
    match opt {
        Opt::Create { title, text } => println!("Processing Opt::Create with title = {:?}, text = {:?}", title, text),
        Opt::Edit { .. } => println!("Processing Opt::Edit with {:?}", opt),
        Opt::List => println!("Processing Opt::List"),
    }
}
