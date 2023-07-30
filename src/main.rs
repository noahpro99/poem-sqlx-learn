use envconfig::Envconfig;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use std::error::Error;
mod endpoints;
mod models;
mod config;

struct Api;
#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let config = config::Config::init_from_env().expect("Failed to read config from env");
    let pool = sqlx::PgPool::connect(&config.database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_services = (
        endpoints::book::BookApi { pool },
        endpoints::users::UsersApi,
    );

    let api_service =
        OpenApiService::new(api_services, "MeetAIApi", "1.0").server("http://localhost:3000/api");
    let explorer = api_service.openapi_explorer();
    let swagger = api_service.swagger_ui();
    let redoc = api_service.redoc();
    let rapidoc = api_service.rapidoc();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", explorer)
                .nest("/redoc", redoc)
                .nest("/rapidoc", rapidoc)
                .nest("/swagger", swagger),
        )
        .await?;
    Ok(())
}
