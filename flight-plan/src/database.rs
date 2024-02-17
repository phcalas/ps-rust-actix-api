use diesel::backend::Backend;
use serde::{Deserialize, Serialize};
use env_logger::Env;
use diesel::prelude::*;
use diesel::connection::Connection;
use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::r2d2::{ManageConnection, Pool, Error as R2D2Error, ConnectionManager};
use actix_web::web;
use actix_web::middleware::Logger;

use diesel::row::NamedRow;

use crate::models::{User, FlightPlan};
#[warn(unused_imports)]
use uuid::Uuid;
use crate::schema::users::dsl::users;

//pub type DbPool = diesel_connection::ConnectionPool;
//r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn create_user(pool: web::Data<Pool<ConnectionManager<PgConnection>>>, user: User) -> Result<String, Box<Error>> {
    let api_key = Uuid::new_v4().as_simple().to_string();    
    //let mut statement = connection.prepare("INSERT INTO users (full_name, api_key) VALUES (?, ?, ?, ?)")?;
    //let _ = statement.execute((user.name, api_key.clone()))?;

    //println!("User: {:?}", user);
    log::error!("User: {:?}", user);
    //ServerError::log("Username not found.".to_string())
    //env_logger::fmt("User: {:?}", user);

    // Add API ke to the user
    let mut u = user.clone();
    u.api_key = api_key.clone();
    let new_users = vec![u];

    let mut connection = web::Data::from(pool).get().unwrap();
    diesel::insert_into(users)
        .values(&new_users)
        .execute(&mut connection)
        .expect("TODO: panic message");
    Ok(api_key)
}

/*
pub fn get_user(api_key: String) -> Result<Option<User>, Box<dyn Error>> {
    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM users WHERE api_key = ?")?;
    let query_result = statement.query_map([&api_key], |row| {
        Ok(User {
            name: row.get(1)?,
            api_key: row.get(4)?
        })
    })?;

    let mut user: Option<User> = None;
    for api_user in query_result {
        user = Some(api_user.unwrap());
    }

    Ok(user)
}

pub fn get_all_flight_plans() -> Result<Option<Vec<FlightPlan>>, Box<dyn Error>> {
    let mut flight_plan_list : Vec<FlightPlan> = Vec::new();

    let mut connection = get_database_connection()?;
    let statement = sql_query("SELECT * FROM flight_plan").load(&mut connection);
    let query_result = get_result
    statement.map(|row| {
        Ok(FlightPlan {
            flight_plan_id: row.get(1)?,
            altitude: row.get(2)?,
            airspeed: row.get(3)?,
            aircraft_identification: row.get(4)?,
            aircraft_type: row.get(5)?,
            arrival_airport: row.get(6)?,
            departing_airport: row.get(7)?,
            flight_type: row.get(8)?,
            departure_time: row.get(9)?,
            estimated_arrival_time: row.get(10)?,
            route: row.get(11)?,
            remarks: row.get(12)?,
            fuel_hours: row.get(13)?,
            fuel_minutes: row.get(14)?,
            number_onboard: row.get(15)?
        })
    })?;

    for plan in query_result {
        flight_plan_list.push(plan?);
    }

    match flight_plan_list.len() > 0 {
        true => {
            Ok(Some(flight_plan_list))
        }
        false => {
            Ok(None)
        }
    }
}

pub fn get_flight_plan_by_id(plan_id: String) -> Result<Option<FlightPlan>, Box<dyn Error>> {
    let connection = get_database_connection()?;
    let mut statement = connection.prepare("SELECT * FROM flight_plan WHERE flight_plan_id = ?1")?;
    let query_result = statement.query_map([&plan_id], |row| {
        Ok(FlightPlan {
            flight_plan_id: row.get(1)?,
            altitude: row.get(2)?,
            airspeed: row.get(3)?,
            aircraft_identification: row.get(4)?,
            aircraft_type: row.get(5)?,
            arrival_airport: row.get(6)?,
            departing_airport: row.get(7)?,
            flight_type: row.get(8)?,
            departure_time: row.get(9)?,
            estimated_arrival_time: row.get(10)?,
            route: row.get(11)?,
            remarks: row.get(12)?,
            fuel_hours: row.get(13)?,
            fuel_minutes: row.get(14)?,
            number_onboard: row.get(15)?
        })
    })?;

    let mut flight_plan: Option<FlightPlan> = None;

    for plan in query_result {
        flight_plan = Some(plan?);
        break;
    }

    Ok(flight_plan)

}

pub fn delete_flight_plan(plan_id: String) -> Result<bool, Box<dyn Error>> {
    let mut successful = false;
    let connection = get_database_connection()?;
    let mut statement = connection.build_transaction("DELETE FROM flight_plan WHERE flight_plan_id = ?1")?;
    let query_result = statement.execute([&plan_id])?;
    if query_result > 0 {
        successful = true;
    }
    Ok(successful)
}

pub fn insert_flight_plan(flight_plan: FlightPlan) -> Result<(), Box<dyn Error>> {
    let connection = get_database_connection()?;
    let new_flight_plan_id = Uuid::new_v4().simple().to_string();

    let mut statement = connection.build_transaction("INSERT INTO flight_plan (flight_plan_id, altitude, airspeed, aircraft_identification, \
                                                         aircraft_type, arrival_airport, departing_airport, flight_type, departure_time, \
                                                         estimated_arrival_time, route, remarks, fuel_hours, fuel_minutes, number_onboard) \
                                                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;
    let _ = statement.execute((new_flight_plan_id, flight_plan.altitude, flight_plan.airspeed, flight_plan.aircraft_identification,
                                      flight_plan.aircraft_type, flight_plan.arrival_airport, flight_plan.departing_airport, flight_plan.flight_type,
                                      flight_plan.departure_time, flight_plan.estimated_arrival_time, flight_plan.route, flight_plan.remarks,
                                      flight_plan.fuel_hours, flight_plan.fuel_minutes, flight_plan.number_onboard))?;
    Ok(())
}

pub fn update_flight_plan(flight_plan: FlightPlan) -> Result<bool, Box<dyn Error>> {
    let connection = get_database_connection()?;
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
