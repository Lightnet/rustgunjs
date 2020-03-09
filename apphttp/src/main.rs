//https://actix.rs/docs/websockets/

extern crate actix;
extern crate actix_web;
extern crate actix_web_actors;
extern crate actix_files;
extern crate serde_json;
extern crate rand;

use serde_json::{Value};

use actix::{Actor, StreamHandler};
//use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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

//===============================================
// == Gun start setup
//===============================================
struct Gun{
    #[allow(dead_code)]
    dup:Dup,
}

struct DupOpt{
    #[allow(dead_code)]
    max:i32,
    #[allow(dead_code)]
    age:i32
}

struct Dup{
    s: Vec<Value>,
    #[allow(dead_code)]
    opt: DupOpt
}

impl Gun{

    #[allow(dead_code)]
	fn get(&self){
		println!("get");
    }

	#[allow(dead_code)]
	fn put(&self){
		println!("put");
    }

	#[allow(dead_code)]
	fn dup(&self){
		println!("dup");
    }
    
    #[allow(dead_code)]
    fn test(&self){
		println!("impl gun test");
	}
}

//default setup variable
impl Default for Gun{
    fn default() -> Gun{
        Gun {
            dup: Dup{ 
                s: vec![],
                opt:  DupOpt{max:1000,age:9000} 
            }
        }
    }
}


impl Dup{

    #[allow(dead_code)]
    fn check(&self){
		println!("check");
    }

    #[allow(dead_code)]
    fn track(&self){
		println!("track");
    }

    #[allow(dead_code)]
    fn random(&self)-> String{
        //https://www.reddit.com/r/rust/comments/2s9qzh/how_i_can_generate_random_string_in_rust/
        //https://stackoverflow.com/questions/59553586/how-do-i-generate-a-string-of-random-ascii-printable-characters-with-a-specific
        //let rand_string: String = thread_rng()
            //.sample_iter(&Alphanumeric)
            //.take(32)
            //.collect();
        //println!("{}", rand_string);
        //rand_string
        thread_rng().sample_iter(&Alphanumeric)
                .take(32)
                .collect() //return random 32 letters and numbers
    }
}

//===============================================
// Gun End setup
//===============================================
struct GunWs{
	gun: Gun //setup gun
}
//
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
                self.gun.test();//gun fn test

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
//http response and request
async fn gunws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(GunWs {gun: Gun::default()}, &req, stream);//setup websocket
    /*
    let resp = ws::start(GunWs {gun: Gun {
            dup: Dup{ 
                opt:  DupOpt{max:1000,age:9000} 
            }
        }
    }, &req, stream);//setup websocket
    */
    println!("{:?}", resp);
    resp //return response
}

// htpp url
//async fn greet(req: HttpRequest) -> impl Responder {
    //let name = req.match_info().get("name").unwrap_or("World");
    //format!("Hello {}!", &name)
//}

//use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;


//main entry point
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("web host server init...");

    let gun = Gun::default();
    let randstring = gun.dup.random();
    println!("{}",randstring);

    let _timer_out = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        println!("timeout...");
    });

    HttpServer::new(|| {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/ws/", web::get().to(indexws)) //default websocket
            .route("/gun/", web::get().to(gunws)) //gun websocket
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