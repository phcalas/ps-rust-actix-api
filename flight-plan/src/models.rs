use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};

use crate::schema::{users, flightplans};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name = flightplans)]
pub struct FlightPlan {
    pub flight_plan_id: String,
    pub altitude: i32,
    pub airspeed: i32,
    pub aircraft_identification: String,
    pub aircraft_type: String,
    pub arrival_airport: String,
    pub departing_airport: String,
    pub flight_type: String,
    pub departure_tim: String,
    pub estimated_arrival_time: String,
    pub route: String,
    pub remarks: String,
    pub fuel_hours: i32,
    pub fuel_minutes: i32,
    pub number_onboard: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub api_key: String
}
