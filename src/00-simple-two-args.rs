use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    title: String,
    cutoff: i32,
}

fn main() {
    let opt = Opt::from_args();
    println!("title = {}", opt.title);
    println!("cutoff = {}", opt.cutoff);
}
