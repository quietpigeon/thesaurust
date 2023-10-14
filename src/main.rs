use structopt::StructOpt;
use thesaurust::{fetch_response, protocol::*};

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    tokio::task::spawn_blocking(|| {
        let data = fetch_response(args).unwrap();
        let results: Vec<Thesaurus> = serde_json::from_value(data).unwrap();
        let result = &results[0];
        let meanings = result.meanings.as_ref().unwrap();
        println!("{}", meanings[0].partOfSpeech);

        //println!("{:#?}", result);
    })
    .await
    .expect("Task panicked");
}
