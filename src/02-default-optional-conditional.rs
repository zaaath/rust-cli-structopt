use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcli", about = "Rust CLI examples with StructOpt")]
struct Opt {
    #[structopt(long)]
    title: String,

    #[structopt(short, default_value = "420")]
    cutoff: i32,

    #[structopt(short, required_if("cutoff", "0"))]
    amplification: Option<i32>,

    #[structopt(short = "m", long = "message")]
    text: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
}
