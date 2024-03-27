use crate::database;
use crate::database::DbPool;
#[warn(unused_imports)]
use crate::models::{FlightPlan, User};
#[warn(unused_imports)]
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, ManageConnection, Pool};
use diesel::PgConnection;
#[warn(unused_imports)]
use log::{debug, error, info, log_enabled, warn, Level};

#[post("/api/v1/admin/user/create")]
pub async fn new_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user: web::Json<User>,
) -> impl Responder {
    match database::create_user(pool, user.into_inner().clone()) {
        Ok(api_key) => return HttpResponse::Ok().body(api_key),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/v1/flightplan")]
pub async fn get_all_flight_plans(pool: web::Data<DbPool>) -> impl Responder {
    debug!("Get flights...");
    match database::get_all_flight_plans(pool) {
        Ok(flight_plan_list) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(flight_plan_list);
        }
        Err(e) => {
            return HttpResponse::NoContent().body(format!(
                "There are no flight plans filed with this system {}",
                e.to_string()
            ));
        }
    }
}

#[get("/api/v1/flightplan/{flight_plan_id}")]
pub async fn get_flight_plan_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let flight_plan_id = path.into_inner().clone();

    debug!("Get flight {}", flight_plan_id.to_string());

    let _ = match database::get_flight_plan_by_id(pool, &flight_plan_id) {
        Ok(flight_plan) => match flight_plan {
            Some(flight_plan_from_db) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(flight_plan_from_db)
            }
            None => {
                return HttpResponse::NotFound().body(format!(
                    "There is not any flight plan with id {}",
                    flight_plan_id
                ))
            }
        },
        Err(e) => {
            return HttpResponse::NotFound().body(format!(
                "There is not any flight plan with id {} with error {}",
                flight_plan_id, e
            ))
        }
    };
}

#[delete("/api/v1/flightplan/{flight_plan_id}")]
pub async fn delete_flight_plan_by_id(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let flight_plan_id = path.into_inner();

    debug!("Delete flight {}", flight_plan_id.to_string());

    match database::delete_flight_plan(pool, &flight_plan_id) {
        Ok(successful) => {
            if successful {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().body(format!(
                    "There is not any flight plan with id {}",
                    flight_plan_id
                ))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/api/v1/flightplan")]
pub async fn create_flight_plan(
    flight_plan: web::Json<FlightPlan>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    debug!("Create flight {}", flight_plan.flight_plan_id.to_string());

    match database::insert_flight_plan(pool, &flight_plan) {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/api/v1/flightplan")]
pub async fn update_flight_plan(
    flight_plan: web::Json<FlightPlan>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    debug!("Update flight {}", flight_plan.flight_plan_id.to_string());

    let updated_flight_plan = flight_plan.into_inner();
    match database::update_flight_plan(pool, &updated_flight_plan) {
        Ok(succeeded) => {
            if succeeded {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().body(format!(
                    "There is not any flight plan with id {}",
                    updated_flight_plan.flight_plan_id
                ))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
