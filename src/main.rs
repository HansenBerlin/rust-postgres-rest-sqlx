mod model;
mod schema;
mod handler;
mod prints_controller;
mod files_controller;
mod users_controller;
mod query_service;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use utoipa_swagger_ui::SwaggerUi;
use model::*;
use schema::*;
use users_controller::*;
use files_controller::*;
use utoipa::{OpenApi};


pub struct AppState {
    db: Pool<Postgres>,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
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
            get_file,
            get_private_files,
            get_public_files,
            create_file,
            delete_file,
            edit_file,
            get_user_id_by_mail,
            create_user
        ),
        components(schemas(
            UpdateFile,
            CreateFile,
            IdSchema,
            CreateUser,
            FileResponse
        ))
    )]
    struct ApiDoc;


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method()
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}



