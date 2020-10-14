use actix_web::{web, HttpRequest, Responder};
use futures::{StreamExt, TryStreamExt};
use actix_multipart::Multipart;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;

#[allow(unused_imports)]
use std::sync::Mutex;
use std::io::Write;

use super::super::utils;


#[derive(Serialize)]
struct UploadResponse {
    success: bool,
    attachments: Vec<String>
}
struct UserEntry {
    token: String,
    domains: Vec<String>
}

#[derive(Serialize)]
impl Serialize for UserEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let mut s = serializer.serialize_struct("UserEntry", 2)?;
        s.serialize_field("domains", &self.domains)?;
        s.serialize_field("token", &self.token)?;
    }
}

pub async fn upload(mut payload: Multipart, req: HttpRequest, engine_data: web::Data<Mutex<utils::mongo::MongoEngine>>) -> impl Responder {
    let mut names: Vec<String> = Vec::new();
    let mut domains: Vec<String> = vec![String::from("http://sharex.naminginprogress.com/")];
    let mut status = false;
    //mongo_engine.insert("accounts", doc!{"cake": "yum"}).await.unwrap();

    // Handle Headers
    let headers = req.headers();
    if headers.contains_key("Authorization") == false {
        return serde_json::to_string(&UploadResponse{
            success: status,
            attachments: names
        })
    }

    // FALLBACK URL EVALUATION
    // ! PENDING DEPRECATION !
    if headers.contains_key("domains") == true {
        let domains_pre_parse = headers.get("domains")
            .unwrap()
            .to_str()
            .unwrap();
        let slices: Vec<String> = domains_pre_parse.split(" ")
            .map(|s| s.to_string())
            .collect();
        domains.extend(slices)
    }
    // ! END OF DEPRECATION  !


    let token = headers.get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    if dotenv!("MONGO") == "true" {
        let mongo_engine = engine_data.lock().unwrap();
        let entries = mongo_engine.query("accounts", "token", &token).await.unwrap();
        if entries.len() == 0 || token == "" {
            return serde_json::to_string(&UploadResponse{
                success: status,
                attachments: names
            })
        }
        entries.first()
            .unwrap()
            .get("domains")
            .serialize(serializer)
    }

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().type_();
        if content_type == "image" {
            let file_name = utils::random::gen_sequence(8);
            names.push(file_name.clone());
            let mut f = web::block(move || std::fs::File::create(format!("./images/{}.png", file_name)))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
            }
        } else {
            let file_name = utils::random::gen_sequence(8);
            let mut f = web::block(move || std::fs::File::create(format!("./files/{}.txt", file_name)))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap();
            }
        }
        status = true;
    }

    names = names.iter().map(|x| format!("{}{}",
        domains.choose(&mut rand::thread_rng()).unwrap(), x)).collect();

    let upstruct = UploadResponse{
        success: status,
        attachments: names
    };
    serde_json::to_string(&upstruct)
}