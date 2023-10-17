use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_admin_dashboard().await;

    // assert
    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn logout_clears_session_state() {
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

    // act - part 3 - logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // act - part 4 - follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out</i></p>"#));

    // act - part 5 - attempt to load admin panel
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
