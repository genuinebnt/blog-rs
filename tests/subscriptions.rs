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

    // Verify the user was actually inserted into the database
    let saved = sqlx::query!(
        "SELECT email, name FROM users WHERE email = $1",
        "genuine.basilnt@gmail.com"
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "genuine.basilnt@gmail.com");
    assert_eq!(saved.name, "genuine");
}

#[tokio::test]
pub async fn subscribe_returns_422_for_invalid_body() {
    let app = utils::spawn_app().await;
    let address = app.address;
    let client = reqwest::Client::new();

    let invalid_bodies = vec![
        (
            "name=genuine",
            "Failed to deserialize form body: missing field `email`",
        ),
        (
            "email=genuine.basilnt@gmail.com",
            "Failed to deserialize form body: missing field `name`",
        ),
        ("", "Failed to deserialize form body: missing field `name`"),
    ];

    for (invalid_body, error_message) in invalid_bodies {
        let response = client
            .post(format!("http://{address}/subscribe"))
            .body(invalid_body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .unwrap();

        assert_eq!(422, response.status().as_u16());
        assert_eq!(error_message, response.text().await.unwrap());
    }
}
