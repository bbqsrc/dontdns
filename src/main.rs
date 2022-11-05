use std::{time::Duration, collections::HashMap};

use poem::{
    handler, http::HeaderMap, listener::TcpListener, middleware::Tracing, post, Body, EndpointExt,
    Route, Server, get, web::Query,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Hello, world!");

    let app = Route::new()
        .at("/nic/update", get(nic_update))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:10053"))
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await?;

    Ok(())
}

#[handler]
async fn nic_update(params: Query<HashMap<String, String>>, headers: &HeaderMap) {
    // format!("hello: {}", name)
    // let body_text = body.into_string().await.unwrap();
    tracing::info!(query = ?params, headers = ?headers, "Hi");
}
