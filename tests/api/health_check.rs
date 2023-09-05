use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
