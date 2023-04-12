use crate::{
    model::FileModel,
    schema::{CreateFileSchema, FilterOptions, UpdateFileSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use crate::model::PrintModel;

#[get("/")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/files")]
pub async fn file_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        FileModel,
        "SELECT * FROM file ORDER by id LIMIT $1 OFFSET $2",
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
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/files/")]
async fn create_file_handler(
    body: web::Json<CreateFileSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        FileModel,
        "INSERT INTO file (fullname, author, downloads, average_rating, sizebytes) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.fullname.to_string(),
        body.author.to_string(),
        body.downloads.to_owned().unwrap_or(0),
        body.average_rating.to_owned().unwrap_or(0.0),
        body.sizebytes,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(json!({"status": "fail","message": "Note with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/files/{id}")]
async fn get_file_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(FileModel, "SELECT * FROM file WHERE id = $1", note_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found", note_id);
            return HttpResponse::NotFound()
                .json(json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/files/{id}")]
async fn edit_file_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateFileSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(FileModel, "SELECT * FROM file WHERE id = $1", note_id)
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
        FileModel,
        "UPDATE file SET fullname = $1, author = $2, downloads = $3, average_rating = $4 WHERE id = $5 RETURNING *",
        body.fullname.to_owned().unwrap_or(note.fullname),
        body.author.to_owned().unwrap_or(note.author),
        body.downloads.to_owned().unwrap_or(note.downloads.unwrap()),
        body.average_rating.unwrap_or(note.average_rating.unwrap()),
        note_id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": message}));
        }
    }
}

#[delete("/files/{id}")]
async fn delete_file_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
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


#[get("/prints")]
pub async fn print_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        PrintModel,
        "select pr.id as id, nozzle_size_mm, bed_temp_celsius, extruder_temp, successful,
            concat(mb.full_name, ' ', m.description) as filament,
            concat(mat_type, '') as filament_type, concat(pb.full_name, ' ', model) as printer, g.id as gcode_id
        from print pr
            left join material m on m.id = pr.material_fk
            left join printer p on p.id = pr.printer_fk
            left join gcode g on g.id = pr.gcode_fk
            left join material_brand mb on mb.id = m.material_brand_fk
            LEFT JOIN printer_brand pb on pb.id = p.printer_brand_fk ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
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
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}



pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(file_list_handler)
        .service(create_file_handler)
        .service(get_file_handler)
        .service(edit_file_handler)
        .service(delete_file_handler)
        .service(print_list_handler);

    conf.service(scope);
}
