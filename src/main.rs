
use actix_web::{web, App, HttpServer, Responder};
use std::collections::HashMap;
use serde_json::json;


// Define the handler function
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

async fn hellos(_name: web::Path<String>) -> impl Responder {
    let mut v: Vec<HashMap<&str, i8>> = Vec::new();
    let mut hm: HashMap<&str, i8> = HashMap::new();
    let mut hm1: HashMap<&str, i8> = HashMap::new();
    hm.insert("shoaib", 6);
    hm.insert("aman", 5);
    hm.insert("ankit", 16);
    hm.insert("name", 87);
    hm1.insert("shoaib", 6);
    hm1.insert("aman", 5);
    hm1.insert("ankit", 16);
    hm1.insert("player", 87);
    for _ in 1..5{
        let mut item: HashMap<&str, i8> = HashMap::new();
        item.insert("name", 87);
        item.insert("shoaib", 6);
        item.insert("aman", 5);
        item.insert("ankit", 16);
        item.insert("player", 87);
        v.push(item);

    }
    v.push(hm);
    v.push(hm1);

    let mut freq : HashMap<&str,i8> = HashMap::new();
    for data in v{
        for (key,_value) in data{
            if let Some(&value) = freq.get(key) {
                freq.insert(key,value+1);
            }
            else{
                freq.insert(key,1);
            }
        }

    }

    let json_value = json!(freq);
    // hm.insert(name, 87);

    // let json_data = serde_json::to_string(freq).unwrap();
    let json_string = serde_json::to_string(&json_value).expect("Failed to serialize to JSON");

     // Serialize the Vec<HashMap<i8, i8>>

    // Respond with the JSON data
    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(json_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            // Define the route and attach the handler
            .service(web::resource("/greet/{name}").route(web::get().to(greet)))
            .service(web::resource("/cargo/{name}").route(web::get().to(hellos)))
    })
    .bind("127.0.0.1:8080")? // Replace with your desired IP address and port
    .run()
    .await
}
