//////////////////////////////////////////
//tokio.spawn

// fn spawn_app() {
//     let server = rusty_back::run().expect("failed to start");

//     //launch server in background
//     let _ = tokio::spawn(server);
//     //server runs concurrently with tasks
// }

/////////////////////////////////////

/// src/routes/heath_check.rs

use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use rusty_back::config::get_configuration;

fn spawn_app() -> String {
    //BIND TO RANDOM PORT. OS picks random port
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind");

    //Get actual port assigned
    let port = listener.local_addr()
        .unwrap()
        .port();

    //start server on new listener
    let server = run(listener).expect("failed to start");
    tokio::spawn(server);

    //return addy for test to use
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .unwrap();
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke `PgConnection::connect` - it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "prof_radon.com");
    assert_eq!(saved.name, "prof radon");
}

#[test::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arange
    let app_address = spawn_app();
    let client = reqwest::client::new();
    let test_case = vec![
        ("name=radon%20hassan", "missing the email"),
        ("email=h4554n.abdul%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_case {
        //Act
        let response = client 
            .post(&format!("{}/subscription", &app_address))
            .header("content-type, "application/x-www-form-urlencoded)
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

}