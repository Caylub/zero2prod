//! tests/health_check.rs

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // No .await and no expect
    spawn_app();
}

// No .await cal, therefor no need for `spawn_app` to be async
// We are also running tests, so it is not worth it to propogate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things
fn spawn_app() {
    let server = run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
