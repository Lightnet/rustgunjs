//https://actix.rs/docs/websockets/

extern crate actix;
extern crate actix_web;
extern crate actix_web_actors;
extern crate actix_files;
extern crate serde_json;


use serde_json::{Value};

use actix::{Actor, StreamHandler};
//use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;

/// Define http actor
struct MyWs;
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}
/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
async fn indexws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

struct GunWs;
impl Actor for GunWs {
    type Context = ws::WebsocketContext<Self>;
}
/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GunWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {

                let data: Value = serde_json::from_str(&text).unwrap();
                println!("data: {}", data["#"]);
                if data["#"] != Value::Null {
                    println!("Found! #");
                }else{
                    println!("Null #");
                }

                ctx.text(text);
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn gunws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(GunWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

// htpp url
//async fn greet(req: HttpRequest) -> impl Responder {
    //let name = req.match_info().get("name").unwrap_or("World");
    //format!("Hello {}!", &name)
//}

//main entry point
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/ws/", web::get().to(indexws))
            .route("/gun/", web::get().to(gunws))
            // static files
            //.service(fs::Files::new("/", "static/").index_file("index.html"))
            //.service(fs::Files::new("/static", ".").show_files_listing())
            //                  new("url" , "dir")
            //.service(fs::Files::new("/", "static/").show_files_listing())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
//https://actix.rs/docs/static-files/