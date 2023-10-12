use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    word: String
}

struct Thesaurus {
    id: String,
    uuid: String,
    src: String,
    section: String,
    target: Vec<String>,
    syns: Vec<String>,
    ants: Vec<String>,
    offensive: bool
}

fn main() {
    let args = Cli::from_args();
    println!("Input: {}", args.word)
}
