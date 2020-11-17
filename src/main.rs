use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use rand::Rng;
use serde::{Deserialize, Serialize};
use log::debug;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    email: String,
}

async fn index(user_id:Identity) -> String {
    use schema::users::dsl::*;
    let connection = establish_connection();

    use diesel::sql_query;
    let results: Vec<Tree> = sql_query("
        WITH RECURSIVE r AS (
            SELECT id, name AS path, 1 AS depth FROM directories WHERE id = 1
            UNION ALL
            SELECT directories.id, r.path || directories.name as path, r.depth + 1 FROM directories INNER JOIN r ON directories.parent = r.id
            )
            SELECT id,path,depth FROM r order by path
    ").load(&connection).expect("error");

    println!("{:?}", results);

    let results = users.load::<User>(&connection).expect("error loading users");
    let user_name = match results.first() {
        Some(user) => user.email.clone(),
        None => "Not Found User".to_string()
    };

    format!(
        "Hello DB Record {}",
        "success"
    )
}

async fn login(params: web::Form<LoginForm>, id: Identity) -> HttpResponse {
    id.remember( params.email.clone() );
    HttpResponse::Found().header("location", "/").finish()
}

async fn authonly(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(id) => HttpResponse::Ok().body( format!("success {}", id) ),
        None => HttpResponse::Found().header("location", "/").finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new( move || { 
        App::new()
        .wrap(middleware::Logger::default())
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&private_key)
                .name("auth-example")
                .secure(false),
        ))
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/authonly").route(web::get().to(authonly)))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
