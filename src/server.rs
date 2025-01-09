use axum::Router;


pub async fn simple_serve_public(router: Router, port: u16, app_name: &str) {
    let app_name = app_name.to_string();
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.expect("failed to install ctrl+C signal handler!");
        shutdown_tx.send(()).expect("send close signal failed!");
    });

    log::info!("{app_name} listen at http://{addr}/");
    log::info!("Local listen at http://127.0.0.1:{port}/");
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            shutdown_rx.await.ok();
            log::info!("{app_name} exits successfully!");
        })
        .await.unwrap();
}

pub async fn simple_serve_local(router: Router, port: u16, app_name: &str) {
    let app_name = app_name.to_string();
    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.expect("failed to install ctrl+C signal handler!");
        shutdown_tx.send(()).expect("send close signal failed!");
    });

    log::info!("Local {app_name} listen at http://127.0.0.1:{port}/");
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            shutdown_rx.await.ok();
            log::info!("{app_name} exits successfully!");
        })
        .await.unwrap();
}