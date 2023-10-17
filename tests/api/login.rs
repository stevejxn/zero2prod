use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // arrange
    let app = spawn_app().await;

    // act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login_body).await;

    // assert
    assert_is_redirect_to(&response, "/login");

    // act - part 2
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // act - part 3
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"Authentication failed"#));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    // arrange
    let app = spawn_app().await;

    // act - part 1 - login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // act - part 2 - follow the redirect
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
