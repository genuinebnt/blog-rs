mod utils;

#[tokio::test]
async fn health_check_works() {
    let address = utils::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{address}/health_check"))
        .send()
        .await
        .unwrap();

    assert_eq!(200, response.status().as_u16());
}
