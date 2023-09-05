use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let body = "name=steve%20jackson&email=steve%40s2j.co.uk";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Faled to fetch saved subscription.");

    assert_eq!(saved.email, "steve@s2j.co.uk");
    assert_eq!(saved.name, "steve jackson");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=test@example.com", "empty name"),
        ("name=test&email=", "empty email"),
        ("name=test&email=this-is-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 when payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=steve%jackson", "missing the email"),
        ("email=steve%40s2j.co.uk", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        // act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
