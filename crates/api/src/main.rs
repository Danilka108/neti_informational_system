use axum::Router;
use sea_orm::{Database, DatabaseConnection};

mod entities;
mod handlers;
mod utils;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = std::env::var("API_HOST").expect("HOST is not set in .env file");
    let port = std::env::var("API_PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    // Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let app = Router::new()
        // .route("/", get(list_posts).post(create_post))
        // .route("/:id", get(edit_post).post(update_post))
        // .route("/new", get(new_post))
        // .route("/delete/:id", post(delete_post))
        // .nest_service(
        //     "/static",
        //     get_service(ServeDir::new(concat!(
        //         env!("CARGO_MANIFEST_DIR"),
        //         "/static"
        //     )))
        //     .handle_error(|error| async move {
        //         (
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             format!("Unhandled internal error: {error}"),
        //         )
        //     }),
        // )
        // .layer(CookieManagerLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
