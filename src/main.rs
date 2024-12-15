pub mod api;
pub mod server;
pub mod model;

use std::sync::Arc;

use server::AppContext;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    use tracing_subscriber::filter::FilterFn;
    use tracing_subscriber::Layer;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG)
                .with_filter(FilterFn::new(|meta| !meta.target().starts_with("hyper::"))),
        )
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let mut args = std::env::args().skip(1);

    let db_path = args.next().expect("Path to DB missing");
    let db_reader = maxminddb::Reader::open_readfile(db_path).unwrap();

    let context = AppContext::new(Some("geoip"), db_reader);
    let app_state = Arc::new(context);

    let listen_addr = "127.0.0.1:9124";
    tracing::info!("Starting server on {}", listen_addr);
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    let app_internal = server::build_app(app_state.clone());
    let main_task = tokio::spawn(server::serve_tcp(app_internal, listener));

    main_task.await.unwrap();
}
