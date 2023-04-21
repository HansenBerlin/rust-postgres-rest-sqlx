use crate::model::UserModel;
use crate::{schema::FilterOptions, AppState, GetIdSchema};
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
pub async fn user_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM user_account LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all note items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "users": notes
    });
    HttpResponse::Ok().json(json_response)
}

#[utoipa::path(responses(
(status = 200, description = "OK, User Uuid", body = GetIdSchema),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)),
params(("mail" = String, Path, description = "User Mail")))]
#[get("/users/{mail}")]
pub async fn get_user_id_by_mail(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mail = path.into_inner();
    let query_result = sqlx::query_as!(
        GetIdSchema,
        "
        SELECT id FROM user_account ua
            LEFT JOIN user_account_mails um ON ua.id = um.user_account_pk
            WHERE um.mail = $1",
        mail
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(_) => {
            let message = format!("User with mail: {} not found", mail);
            HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
        }
    }
}
