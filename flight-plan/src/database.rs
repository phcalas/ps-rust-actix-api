#[warn(unused_imports)]
use log::{debug, error, log_enabled, info, Level, warn};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::connection::Connection;
use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::r2d2::{ManageConnection, Pool, Error as R2D2Error, ConnectionManager};
use actix_web::web;
use actix_web::middleware::Logger;
use diesel::{sql_query, delete};
use diesel::sql_types::Text;

use crate::models::{User, FlightPlan};
use uuid::Uuid;
use crate::models;
use crate::schema::flight_plans::dsl::flight_plans;
use crate::schema::users::*; //{api_key, fullname, username};
use crate::schema::users::dsl::*;
use crate::schema::flight_plans::*;
use crate::schema::flight_plans::dsl::*;

extern crate log;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_user(
    pool: web::Data<DbPool>,
    user: User,
) -> Result<String, Box<Error>> {

    let apikey = Uuid::new_v4().as_simple().to_string();

    debug!("Create user: {:?}", user);

    // Add API ke to the user
    let mut u = user.clone();
    u.api_key = apikey.clone();
    let new_users = vec![u];

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    diesel::insert_into(users)
        .values(&new_users)
        .execute(&mut connection)
        .expect("TODO: panic message");
    Ok(apikey)
}

pub fn get_user(
    pool: web::Data<DbPool>,
    key: &String,
) -> Result<Option<User>, Error> {

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    // SELECT username, fullname, api_key FROM users WHERE api_key = ?
    let data = users
        .filter(api_key.eq(key))
        .select((username, fullname, api_key))
        .load::<User>(&mut connection)?;

    // Confirm we found api_key
    if key.eq(&data[0].api_key) {
        debug!("Found username: {}", data[0].username);
        return Ok(Some(data[0].clone()));
    } else {
        return Ok(None);
    }
}

pub fn get_all_flight_plans(
    pool: web::Data<DbPool>
) -> Result<Vec<FlightPlan>, Error> {

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    let res = flight_plans
        .select((flight_plan_id, altitude,
                 airspeed, aircraft_identification,
                 aircraft_type, arrival_airport,
                 departing_airport, flight_type,
                 departure_time, estimated_arrival_time,
                 route, remarks,
                 fuel_hours, fuel_minutes,
                 number_onboard
        ));

    let res = res
        .load::<FlightPlan>(&mut connection)?;

    return Ok(res);
}

pub fn get_flight_plan_by_id(
    pool: web::Data<DbPool>,
    plan_id: &String,
) -> Result<Option<FlightPlan>, Error> {

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    let data = flight_plans
        .filter(flight_plan_id.eq(plan_id))
        .select((flight_plan_id, altitude,
                 airspeed, aircraft_identification,
                 aircraft_type, arrival_airport,
                 departing_airport, flight_type,
                 departure_time, estimated_arrival_time,
                 route, remarks,
                 fuel_hours, fuel_minutes,
                 number_onboard))
        .load::<FlightPlan>(&mut connection)?;

    debug!("Found flight: {}", plan_id);

    if data.len() > 0 {
        return Ok(Some(data[0].clone()));
    } else {
        return Ok(None);
    }
}

pub fn delete_flight_plan(
    pool: web::Data<DbPool>,
    plan_id: &String,
) -> Result<bool, Error> {
    let mut successful = false;

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    let num_deleted = diesel::delete(flight_plans.filter(flight_plan_id.eq(plan_id))).execute(&mut connection)?;

    if num_deleted > 0 {
        successful = true;
    }
    Ok(successful)
}

pub fn insert_flight_plan(
    pool: web::Data<DbPool>,
    flight_plan: &FlightPlan
) -> Result<FlightPlan, Error> {

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    let mut val = flight_plan.clone();
    val.flight_plan_id = Uuid::new_v4().simple().to_string();

    let _ = diesel::insert_into(flight_plans)
        .values(&val)
        .returning(FlightPlan::as_returning())
        .get_result(&mut connection)?;

    Ok(val.clone())
}

/*
pub fn update_flight_plan(
    pool: web::Data<DbPool>,
    flight_plan: FlightPlan
) -> Result<bool, Box<dyn Error>> {

    // Get DB connexion from Pool
    let mut connection = web::Data::from(pool).get().unwrap();

    let new_flight_plan_id = Uuid::new_v4().simple().to_string();

    let mut statement = connection.build_transaction("UPDATE flight_plan SET altitude = ?, airspeed = ?, aircraft_identification = ?, \
                                                     aircraft_type = ?, arrival_airport = ?, departing_airport = ?, flight_type = ?, \
                                                     departure_time = ?, estimated_arrival_time = ?, route = ?, remarks = ?, fuel_hours = ?, \
                                                     fuel_minutes = ?, number_onboard = ? WHERE flight_plan_id = ?")?;
    let result_count = statement.execute((flight_plan.altitude, flight_plan.airspeed, flight_plan.aircraft_identification,
                               flight_plan.aircraft_type, flight_plan.arrival_airport, flight_plan.departing_airport, flight_plan.flight_type,
                               flight_plan.departure_time, flight_plan.estimated_arrival_time, flight_plan.route, flight_plan.remarks,
                               flight_plan.fuel_hours, flight_plan.fuel_minutes, flight_plan.number_onboard, flight_plan.flight_plan_id))?;
    let mut succeeded =false;
    if result_count > 0 {
        succeeded = true;
    }
    
    Ok(succeeded)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_database_connection() {
        let conn = get_database_connection();
        match conn {
            Ok(res) => println!("Test Ok"),
            _ => panic!("Test KO")
        };
    }
}
*/
