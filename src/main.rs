use thesaurust::fetch_response;
// use thesaurust::protocol::Thesaurus;

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        let results = fetch_response().unwrap();
        let result = Some(&results[0]);
        println!("{:#?}", result);
    })
    .await
    .expect("Task panicked");
}
