use anyhow::Context;
use axum_test::TestServer;

use play::controller::routers;
use play::init_app_state;

#[tokio::test]
async fn test_root() -> anyhow::Result<()> {
    let server = TestServer::new(routers(init_app_state(play::config::init_config(), true).await)).context("sdf")?;
    let response = server.get("/").await;
    assert_eq!(response.status_code(), 200);


    Ok(())
}


#[tokio::test]
async fn test_redis() -> anyhow::Result<()> {
    let server = TestServer::new(routers(init_app_state(play::config::init_config(), true).await))?;
    let response = server.get("/test-redis").await;
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.text(), "testval");
    Ok(())
}
