use std::sync::Mutex;
#[allow(unused_imports)]

use std::path::{ Path, PathBuf };
use actix_web::{web, App, HttpServer, HttpRequest, Responder};

#[allow(unused_imports)]
use futures::{StreamExt, TryStreamExt};

#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions};

use actix_files as fs;

use dotenv;

use serde::{Serialize};
use mongodb::bson::doc;

mod utils;
mod routes;

#[macro_use]
extern crate dotenv_codegen;

#[derive(Serialize)]
struct UploadResponse {
    success: bool,
    attachments: Vec<String>
}

async fn index(_req: HttpRequest) -> impl Responder {
    //let name = req.match_info().get("name").unwrap_or("World");
    format!("
    Hello There!
    This is a work in progress file upload system.

    Contribute on GitHub: https://github.com/Furry/webserver
    Contact me on Discord: Ether#0621
    ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    if Path::new("./images").exists() == false {
        std::fs::create_dir("./images").unwrap();
    }

    if dotenv!("MONGO") == "true" {
        let client_ = utils::mongo::client(dotenv!("CONNECTION_URI")).await.unwrap();
        let db_ = client_.database("file_uploader");
        let mutx_engine = web::Data::new(Mutex::new(utils::mongo::MongoEngine {
            client: client_,
            db: db_
        }));
        HttpServer::new(move || {
            App::new()
                .app_data(mutx_engine.clone())
                .route("/", web::get().to(index))
                .route("/", web::post().to(routes::upload::upload))
                .service(fs::Files::new("/", "./images"))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    } else {
        HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(index))
                .route("/", web::post().to(routes::upload::upload))
                .service(fs::Files::new("/", "./images"))
        })
        .bind(format!("127.0.0.1:{}", dotenv!("PORT")))?
        .run()
        .await
    }
}