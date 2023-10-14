use structopt::StructOpt;
use thesaurust::{fetch_response, protocol::*};

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    tokio::task::spawn_blocking(|| {
        let data = fetch_response(args).unwrap();
        let result = &data[0];
        let meanings = result.meanings.as_ref().unwrap();
        println!("Part of speech: {}", meanings[0].partOfSpeech);
    })
    .await
    .expect("Task panicked");
}
