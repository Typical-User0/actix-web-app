#[macro_use]
extern crate actix_web;

mod database;
mod handlers;
mod templates;

use actix_files::Files;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::var;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;

type DbPool = Pool<Postgres>;

// here everything begins
#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // register template
    let template = templates::generate_template();

    // read not_found_page to string
    let not_found_page = std::fs::read_to_string("templates/404.html.hbs").unwrap();

    // create pool of connections with database
    let pool = PgPoolOptions::new()
        .max_connections(var("MAX_CONNECTIONS_DB").unwrap().trim().parse().unwrap())
        .connect(
            &var("DATABASE_URL")
                .expect("Can't find database environment variable")
                .clone(),
        )
        .await
        .unwrap_or_else(|err| {
            eprintln!("{:?}", err);
            std::process::exit(-1);
        });

    // environment logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // start HttpServer
    HttpServer::new(move || {
        App::new()
            // share data with handlers
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(template.clone()))
            .app_data(Data::new(not_found_page.clone()))
            // register handlers
            .service(handlers::index)
            .service(handlers::sign_up_post)
            .service(handlers::sign_up_get)
            //register static files
            .service(Files::new("/", "./static").prefer_utf8(true))
            // enable logging
            .wrap(middleware::Logger::default())
            // default not found handler
            .default_service(web::route().to(handlers::not_found))
    })
    .bind(format!(
        "{}:{}",
        var("HOSTNAME").unwrap(),
        var("PORT").unwrap()
    ))?
    .run()
    .await?;
    Ok(())
}
