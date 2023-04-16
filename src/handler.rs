use crate::model::{FileSimpleResponseModel, User};
use crate::printscontroller::print_list_handler;
use crate::schema::CreateFilePermissionSchema;
use crate::users_controller::{get_user_id_by_mail, user_list_handler};
use crate::{
    model::{FileExtendedResponseModel, FileResponseModel, UserModel},
    schema::{CreateFileSchema, FilterOptions, GetIdSchema, UpdateFileSchema},
    AppState,
};
use actix_web::http::header::q;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::{from_value, json};
use uuid::Uuid;
//use postgres::{Client, NoTls};
use tokio_postgres::{Client, NoTls};

const DATABASE_URL: &str = "postgresql://admin:password123@localhost:6500/rust_sqlx";

#[get("/")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = Vec<FileResponseModel>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("id" = Uuid, Path, description = "User Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[get("/files/user/{id}")]
pub async fn get_files_by_user_id(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        FileExtendedResponseModel,
        "
select a.*, b.owner from
(select id, fullname, created, sizebytes, downloads, average_rating,
       fpu.roles_pk from file                                                                                                               left join files_per_user fpu on file.id = fpu.files_pk
where fpu.user_account_pk = $1
union
SELECT file.*, files_per_user.roles_pk
FROM file
         JOIN files_per_user ON file.id = files_per_user.files_pk
WHERE file.id IN (
    SELECT files_pk
    FROM files_per_user
    GROUP BY files_pk
    HAVING COUNT(*) = 1
)) a
JOIN
(select file.id, user_name as owner from file
left join files_per_user fpu on file.id = fpu.files_pk
left join user_account ua on ua.id = fpu.user_account_pk
where fpu.roles_pk = 'owner') b
on a.id = b.id LIMIT $2 OFFSET $3",
        id as Uuid,
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
    HttpResponse::Ok().json(notes)
}

#[get("/users-unsafe/{username}")]
async fn get_user_by_id_unsafe(username: web::Path<String>) -> impl Responder {
    let username = username.into_inner();
    let (client, connection) = tokio_postgres::connect(DATABASE_URL, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query = format!(
        "\
    SELECT username, accountno \
    FROM users_account_numbers \
    WHERE username = '{}'",
        username
    );

    let rows = client.query(query.as_str(), &[]).await.unwrap();
    let mut accounts = Vec::new();
    for row in &rows {
        let username: String = row.get(0);
        let account_no: String = row.get(1);
        accounts.push(json!({"username": username, "accountno": account_no}));
    }

    drop(client);
    HttpResponse::Ok().json(accounts)
}

#[get("/users-safe/{username}")]
async fn get_user_by_id_safe(
    username: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let username = username.into_inner();
    let accounts = sqlx::query_as!(
        User,
        "\
        SELECT username, accountno \
        FROM users_account_numbers \
        WHERE username = $1",
        username
    )
    .fetch_all(&data.db)
    .await;

    match accounts {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = FileResponseModel),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
request_body(content = CreateFileSchema, description="all parameters are required"),
)]
#[post("/files")]
async fn create_file(
    body: web::Json<CreateFileSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        FileResponseModel,
        "
WITH inserted_file AS (
    INSERT INTO file (fullname, downloads, average_rating, sizebytes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, fullname, created, sizebytes, downloads, average_rating
), inserted_files_per_user AS (
    INSERT INTO files_per_user (user_account_pk, roles_pk, files_pk)
        VALUES ($5, 'owner', (SELECT id FROM inserted_file))
        RETURNING user_account_pk, roles_pk, files_pk
)
SELECT inserted_file.id, inserted_file.fullname, inserted_file.created, inserted_file.sizebytes,
inserted_file.downloads, inserted_file.average_rating
FROM inserted_file
",
        body.fullname,
        0,
        0.0,
        body.sizebytes,
        body.owner_user_id
    )
    .fetch_one(&data.db)
    .await;

    return match query_result {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest().json(
                    json!({"status": "fail","message": "Note with that title already exists"}),
                );
            }

            HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}))
        }
    };
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = Vec<FileResponseModel>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("id" = Uuid, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[get("/files/{id}/{userid}")]
async fn get_file_by_id(
    path: web::Path<(Uuid, Uuid)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(
        FileSimpleResponseModel,
        "
select a.*, user_name as owner, roles_pk from
    (select id, fullname, created, sizebytes, downloads, average_rating
     from file
     where file.id = $1) a
        left join files_per_user fpu on a.id = fpu.files_pk
        left join user_account ua on ua.id = fpu.user_account_pk
where ua.id = $2",
        note_id.0,
        note_id.1
    )
    .fetch_one(&data.db)
    .await;

    return match query_result {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(_) => {
            let message = format!("Note with ID: {}, {} not found", note_id.0, note_id.1);
            HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
        }
    };
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = Vec<FileResponseModel>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
request_body(content = UpdateFileSchema, description="not all parameters are required"),
params(
("id" = Uuid, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[patch("/files/{id}")]
async fn edit_file(
    path: web::Path<Uuid>,
    body: web::Json<UpdateFileSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(
        FileResponseModel,
        "SELECT * FROM file WHERE id = $1",
        note_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    //let now = Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        FileResponseModel,
        "UPDATE file SET fullname = $1, downloads = $2, average_rating = $3 WHERE id = $4 RETURNING *",
        body.fullname.to_owned().unwrap_or(note.fullname),
        body.downloads.to_owned().unwrap_or(note.downloads.unwrap()),
        body.average_rating.unwrap_or(note.average_rating.unwrap()),
        note_id
    )
    .fetch_one(&data.db)
    .await
    ;

    return match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error","message": message}))
        }
    };
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = Vec<FileResponseModel>),
(status = 404, description = "File not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("id" = Uuid, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[delete("/files/{id}")]
async fn delete_file(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let note_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM file WHERE id = $1", note_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

#[utoipa::path(responses(
(status = 200, description = "OK", body = String),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("mail" = String, Path, description = "User Mail")
))]
#[get("/usersaa/{mail}")]
pub async fn get_user_id_by_mail2(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mail = path.into_inner();
    HttpResponse::Ok().json(format!("hello {}", mail))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(get_files_by_user_id)
        .service(user_list_handler)
        .service(create_file)
        .service(get_file_by_id)
        .service(edit_file)
        .service(delete_file)
        .service(get_user_id_by_mail)
        .service(get_user_by_id_unsafe)
        .service(get_user_by_id_safe)
        .service(print_list_handler);

    conf.service(scope);
}
