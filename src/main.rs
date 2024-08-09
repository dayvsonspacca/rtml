// use std::time::Instant;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}

// fn main() {
//     env_logger::builder()
//         .filter_level(log::LevelFilter::Trace)
//         .init();

//     let now = Instant::now();

//     let view = rtml::view("component.rtml", vec![("phone", "5581999051134")]);

//     println!("Elapsed time: {:?}", now.elapsed());
// }

#[get("/")]
async fn hello() -> impl Responder 
{
    let view = rtml::view("example.rtml", vec![("phone", "5581999051134")]);

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(view)
}
