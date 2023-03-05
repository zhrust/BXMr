// main.rs

mod inv;
#[tokio::main]
async fn main() {
    //git::cli();
    inv::run().await;
}
