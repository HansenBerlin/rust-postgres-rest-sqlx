mod handler;
mod model;
mod schema;
mod printscontroller;
mod users_controller;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

use model::*;
use schema::*;
use handler::*;
use printscontroller::*;
use std::error::Error;
use utoipa::{ openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}, Modify, OpenApi };
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
//#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    #[derive(OpenApi)]
    #[openapi(
    paths(
    get_file_by_id, get_files_by_user_id, create_file, delete_file, edit_file
    ),
    components(
    schemas(
    FileResponseModel, FileExtendedResponseModel, UserModel, PrintModel, UpdateFileSchema, CreateFileSchema, GetIdSchema, CreateFilePermissionSchema
    )
    )
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-doc/openapi.json", ApiDoc::openapi()))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
