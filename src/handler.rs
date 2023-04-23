use crate::users_controller::{get_user_id_by_mail, user_list_handler, create_user};
use crate::files_controller::{create_file, delete_file, edit_file, get_file_by_id, get_file_by_user};

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;


#[get("/")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(get_file_by_user)
        .service(user_list_handler)
        .service(create_file)
        .service(get_file_by_id)
        .service(edit_file)
        .service(delete_file)
        .service(get_user_id_by_mail)
        .service(create_user);
    conf.service(scope);
}
