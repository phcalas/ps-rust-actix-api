use diesel::table;

table! {
    flight_plans(flight_plan_id) {
        flight_plan_id -> VarChar,
        altitude -> Integer,
        airspeed -> Integer,
        aircraft_identification -> VarChar,
        aircraft_type -> VarChar,
        arrival_airport -> VarChar,
        departing_airport -> VarChar,
        flight_type -> VarChar,
        departure_time -> VarChar,
        estimated_arrival_time -> VarChar,
        route -> VarChar,
        remarks -> VarChar,
        fuel_hours -> Integer,
        fuel_minutes -> Integer,
        number_onboard-> Integer,
    }
}

table! {
    users(username) {
        username -> VarChar,
        fullname -> VarChar,
        password -> VarChar,
        api_key -> VarChar,
    }
}