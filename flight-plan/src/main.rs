mod auth;
mod database;
mod endpoints;
mod models;
mod schema;
mod errors;

use std::ops::Deref;
use crate::endpoints::{
    create_flight_plan, delete_flight_plan_by_id, get_all_flight_plans, get_flight_plan_by_id,
    new_user, update_flight_plan,
};

use env_logger::Env;

#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, warn, Level};

use crate::database::DbPool;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{dev::ServiceRequest, web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use bigdecimal::ToPrimitive;
use config::Config as ConfigFile;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
/// Result<PooledConnection, PoolError>

pub fn get_connection_pool(
    conn_spec: String, // Url of the connection
    app_name: String,  // App name associated to the connection
    max_conn: u32,     // Max connexion to DB
) -> DbPool {
    // Add application name to the connection string
    let mut conn_spec = String::from(conn_spec);
    conn_spec.push('?');
    conn_spec.push_str(&"application_name=");
    conn_spec.push_str(app_name.as_str());
    conn_spec.push_str(&"&sslmode=require");

    // Create ConnectionManager with URL and params
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);

    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .max_size(max_conn)
        .build(manager)
        .expect("Could not build connection pool")
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.deref().clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err((AuthenticationError::from(config).into(), req))
            }
        },
        Err(_) => Err((AuthenticationError::from(config).into(), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = ConfigFile::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let certificate_key = settings.get_string("CERTIFICATE_KEY").unwrap();
    let certificate = settings.get_string("CERTIFICATE").unwrap();
    let conn_spec = settings
        .get_string("DATABASE_URL")
        .expect("DATABASE_URL should be set");
    let app_name = settings
        .get_string("APPLICATION_NAME")
        .unwrap_or("Actix Flight-Plan Demo".to_string());
    let num_workers = settings
        .get_int("NUM_WORKERS")
        .unwrap_or(2)
        .to_usize()
        .expect("NUM_WORKERS >= 0");
    let max_connection = settings
        .get_int("MAX_CONN")
        .unwrap_or(10)
        .to_u32()
        .expect("MAX_CONN >= 0");
    let log_level = settings
        .get_string("LOG_LEVEL")
        .unwrap_or("info".to_string());

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(certificate_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(certificate).unwrap();

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = get_connection_pool(conn_spec, app_name, max_connection);

    env_logger::init_from_env(Env::default().default_filter_or(&log_level));
    info!("Log level: {}", &log_level);

    // Get Pool state for information
    let state = pool.state();
    info!("Pool state {:?}", state);

    HttpServer::new(move || {
        let middleware = HttpAuthentication::bearer(validator);
        let pool = pool.clone();
        App::new()
            .app_data(web::Data::new(pool))
            .service(get_flight_plan_by_id)
            .service(get_all_flight_plans)
            .service(delete_flight_plan_by_id)
            .service(create_flight_plan)
            .service(update_flight_plan)
            .service(new_user)
            .wrap(middleware)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_method()
                    .allow_any_origin()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .bind_openssl("0.0.0.0:3001", builder)?
    .workers(num_workers)
    .run()
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::web::Data;
    use std::sync::Arc;

    #[test]
    fn test_database_get_all_flights() {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
        let pool =
            get_connection_pool(conn_spec, "UT test_database_get_all_flights".to_string(), 1);

        for i in 0..100 {
            let x = crate::database::get_all_flight_plans(Data::new(pool.clone()));
            match x {
                Ok(res) => println!("Test Ok ({})", res.len()),
                _ => panic!("Test KO"),
            }
        }
    }

    #[test]
    fn test_database_get_no_flight() {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
        let pool = get_connection_pool(conn_spec, "UT test_database_get_no_light".to_string(), 1);

        for i in 0..100 {
            let x = crate::database::get_flight_plan_by_id(
                Data::new(pool.clone()),
                &"unknown-flight".to_string(),
            );
            match x {
                Ok(None) => println!("Test Ok (no flight)"),
                _ => panic!("Test KO"),
            }
        }
    }

    #[test]
    fn test_database_get_one_flight() {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
        let pool = get_connection_pool(conn_spec, "UT test_database_get_no_light".to_string(), 1);

        for i in 0..100 {
            let x = crate::database::get_flight_plan_by_id(
                Data::new(pool.clone()),
                &"c20a768f3f464844a2cf8f4379247ff1".to_string(),
            );
            match x {
                Ok(flight) => println!("Test Ok ({} found)", flight.unwrap().flight_plan_id),
                _ => panic!("Test KO"),
            }
        }
    }
}
