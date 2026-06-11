use cellkit_actions_api::{app, config::AppConfig};
use tower::ServiceBuilder;
use vercel_runtime::{axum::VercelLayer, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let service = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(app(config));

    vercel_runtime::run(service).await
}
