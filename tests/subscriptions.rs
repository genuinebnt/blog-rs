mod utils;

#[tokio::test]
pub async fn subscribe_returns_200_for_valid_form() {
    let app = utils::spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();
    let body = "name=genuine&email=genuine.basilnt@gmail.com";

    let response = client
        .post(format!("http://{address}/subscribe"))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .unwrap();
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
pub async fn subscribe_returns_400_for_invalid_body() {
    let app = utils::spawn_app().await;
    let address = app.address;
    let client = reqwest::Client::new();

    let invalid_bodies = vec![
        ("name=genuine", "missing email"),
        ("email=genuine.basilnt@gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in invalid_bodies {
        let response = client
            .post(format!("http://{address}/subscribe"))
            .body(invalid_body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .unwrap();

        assert_eq!(400, response.status().as_u16());
        assert_eq!(error_message, response.text().await.unwrap());
    }
}
