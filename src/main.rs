use std::sync::Mutex;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

use tera::{Tera, Context};

use std::fs::File;
use std::io::Read;
use serde_json;

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


#[get("/reset")]
async fn reset(tera: Data<Tera>, data: Data<AppStateCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter = 0;

    log::info!("Reset Counter Value: {}", *counter);
    let mut reset_context = Context::new();
    reset_context.insert("counter_value", &*counter);
    HttpResponse::Ok().body(tera.render("components/counter.html", &reset_context).unwrap())
}


#[get("/ratings")]
async fn ratings(tera: Data<Tera>) -> impl Responder {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join("data/someStuff.json");
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json_data: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let restaurant_ratings = json_data.as_object().unwrap();

    let mut ratings_context = Context::new();
    for (key, value) in restaurant_ratings.iter() {
        ratings_context.insert(key, value);
    }

    HttpResponse::Ok().body(tera.render("components/ratings.html", &ratings_context).unwrap())
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("Starting Server");

    let tera = Data::new(Tera::new("./templates/**/*.html").unwrap());
    let counter = Data::new(AppStateCounter {
        counter: Mutex::new(0)
    });
    log::info!("Server started at http://0.0.0.0:8000");
    HttpServer::new( move || {
        App::new()
        .app_data(tera.clone())
        .app_data(counter.clone())
        .service(actix_files::Files::new("/static", "./static").show_files_listing())
        .service(home)
        .service(increment)
        .service(decrement)
        .service(reset)
        .service(ratings)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

