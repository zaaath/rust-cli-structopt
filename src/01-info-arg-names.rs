use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcli", about = "Rust CLI examples with StructOpt")]
struct Opt {
    #[structopt(long)]
    title: String,

    #[structopt(short, help = "Cutoff frequency")]
    cutoff: i32,

    #[structopt(short = "m", long = "message")]
    text: String
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
}
