use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use rand::Rng;

async fn index() -> String {
    format!(
        "Hello {}",
        "world".to_string()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new( move || { 
        App::new()
        .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
