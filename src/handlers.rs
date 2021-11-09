use super::database;
use super::database::models::User;
use crate::DbPool;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
pub use database::UserCreationResult;
use handlebars::Handlebars;
use std::collections::HashMap;

mod validators;

#[get("/")]
/// Main page
pub async fn index(templates: web::Data<Handlebars<'_>>) -> HttpResponse {
    // context for rendering template
    let context: HashMap<_, _> = [
        ("username", "\"some_future_username_from_database\""),
        ("title", "main page"),
    ]
    .iter()
    .cloned()
    .collect();

    // render body
    let body = templates.render("index", &context).unwrap_or_else(|err| {
        eprintln!("{}", err);
        String::from("<h1>An error occurred!</h1>")
    });
    // returning html page
    HttpResponse::Ok().content_type("text/html").body(body)
}

/// default not found page
pub async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .body("<h1>Page was not found</h1>")
        .with_status(StatusCode::NOT_FOUND)
}

#[post("signup")]
/// handler for post request on /signup url
pub async fn sign_up_post(
    form: web::Form<User>,
    pool: web::Data<DbPool>,
    templates: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // creating instance of User struct
    let user = User::new(
        form.username().clone(),
        form.password().clone(),
        form.email().clone(),
    );

    // getting result from database validator
    let result = database::add_user(&user, &pool).await;

    // getting message from handlers validators
    let context: HashMap<&str, &str> =
        HashMap::from([("message", validators::validate_user_creation_result(result))]);

    let body = templates
        .render("signup", &context)
        .unwrap_or_else(|_| String::from("<h1>An error occurred!</h1>"));

    // returning html page
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("signup")]
/// handler for get request on /signup
pub async fn sign_up_get(templates: web::Data<Handlebars<'_>>) -> HttpResponse {
    // empty context (no need in it, page is static)
    let context: HashMap<i32, i32> = HashMap::with_capacity(0);
    let body = templates.render("signup", &context).unwrap();
    HttpResponse::Ok().body(body)
}

//TODO: Login page
// #[get("login")]
// pub async fn login_get() {}
// #[post("login")]
// pub async fn login_post() {}
