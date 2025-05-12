#[tokio::main]
async fn main() {
    let app = blog::startup::router()
        .await
        .expect("Failed to create router");
}
