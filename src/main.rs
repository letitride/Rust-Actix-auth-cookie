use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use rand::Rng;
use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    email: String,
}

async fn index(id:Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
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
