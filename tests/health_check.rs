mod utils;

#[tokio::test]
async fn health_check_works() {
    let app = utils::spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{address}/health_check"))
        .send()
        .await
        .unwrap();

    assert_eq!(200, response.status().as_u16());

    sqlx::query!("SELECT 1 as test_column")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to execute query");
}
