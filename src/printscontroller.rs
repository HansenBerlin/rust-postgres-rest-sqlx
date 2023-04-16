use crate::model::PrintModel;
use crate::{schema::FilterOptions, AppState};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

#[get("/prints/{id}")]
pub async fn print_list_handler(
    path: web::Path<Uuid>,
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let file_id = path.into_inner();
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
            LEFT JOIN printer_brand pb on pb.id = p.printer_brand_fk
            left join file f on f.id = g.file_pk
        where f.id = $1
        ORDER by id LIMIT $2 OFFSET $3",
        file_id as Uuid,
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
        "prints": notes
    });
    HttpResponse::Ok().json(json_response)
}
