use actix_web::{get, post, put, delete, web::{Data, Path, Json, ServiceConfig}, Responder, HttpResponse};
// use mongodb::bson::oid::ObjectId;
use super::{models::TodoListData, repository::MongoRepo};

#[get("/todolist/{id}")]
async fn get_todolist(db: Data<MongoRepo>, param: Path<String>) -> impl Responder {
    let id = param.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID Salah!");
    }
    let data_detail = db.get_detail_data(&id).await;
    match data_detail {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/todolist")]
async fn create_todolist(db: Data<MongoRepo>, body_json: Json<TodoListData>) -> impl Responder {
    let data = TodoListData {
        // id: None,
        title: body_json.title.to_owned(),
        job: body_json.job.to_owned()
    };
    let data_detail = db.create_data(data).await;
    match data_detail {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }

}

#[put("/todolist/{id}")]
async fn update_todolist(db: Data<MongoRepo>, param: Path<String>, body_json: Json<TodoListData>) -> impl Responder {
    let id = param.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID Salah!");
    }
    let data = TodoListData {
        // id: Some(ObjectId::parse_str(&id).unwrap()),
        title: body_json.title.to_owned(),
        job: body_json.job.to_owned()
    };
    let update_result = db.update_data(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_data_info = db.get_detail_data(&id).await;
                return match updated_data_info {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("Tidak ditemukan data dengan ID tersebut!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/todolist/{id}")]
async fn delete_todolist(db: Data<MongoRepo>, param: Path<String>) -> impl Responder {
    let id = param.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID Salah!");
    };
    let delete_result = db.delete_data(&id).await;
    match delete_result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Data berhasil dihapus!");
            } else {
                return HttpResponse::NotFound().json("Tidak ditemukan data dengan ID tersebut!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_todolist)
        .service(create_todolist)
        .service(update_todolist)
        .service(delete_todolist);
}
