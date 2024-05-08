use std::sync::Mutex;

use actix_web::{get, App, web::Data, HttpResponse, HttpServer, Responder};

use tera::{Tera, Context};

struct AppStateCounter {

    counter: Mutex<i32>

}

#[get("/")]
async fn home(tera: Data<Tera>, data: Data<AppStateCounter>) -> impl Responder {

    let counter = data.counter.lock().unwrap();

    let mut home_context = Context::new();

    home_context.insert("counter_value", &*counter);

    HttpResponse::Ok().body(tera.render("index.html", &home_context).unwrap())

}


#[get("/increment")]
async fn increment(tera: Data<Tera>, data: Data<AppStateCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    log::info!("Incremented Counter Value: {}", *counter);
    let mut increment_context = Context::new();
    increment_context.insert("counter_value", &*counter);
    HttpResponse::Ok().body(tera.render("components/counter.html", &increment_context).unwrap())

}


#[get("/decrement")]
async fn decrement(tera: Data<Tera>, data: Data<AppStateCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter -= 1;

    log::info!("Decremented Counter Value: {}", *counter);
    let mut decrement_context = Context::new();
    decrement_context.insert("counter_value", &*counter);
    HttpResponse::Ok().body(tera.render("components/counter.html", &decrement_context).unwrap())
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("Starting Server");

    let tera = Data::new(Tera::new("./templates/**/*.html").unwrap());
    let counter = Data::new(AppStateCounter {
        counter: Mutex::new(0)
    });
    log::info!("Server started at 0.0.0.0:8000");
    HttpServer::new( move || {
        App::new()
        .app_data(tera.clone())
        .app_data(counter.clone())
        .service(actix_files::Files::new("/static", "./static").show_files_listing())
        .service(home)
        .service(increment)
        .service(decrement)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

