use axum::{
    extract::Path,
    routing::get,
    Json,
};
use shuttle_axum::axum::Router; // Import Router from shuttle_axum::axum
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use http::{Method, HeaderValue};
use tower_http::cors::AllowOrigin;
use tokio::fs;
use std::io;

mod file;
mod counter;
mod countdown;


async fn hello_world() -> Json<Vec<String>> {
    let mut files = print_files_in_root_dir().await;

    Json(files)
}



async fn get_all_names() -> Json<Vec<(String, String)>> {
    let res = file::get_all_names().await;
    
    Json(res)
}

#[derive(Deserialize)]
struct Params {
    uuid: u64
}

async fn get_timetable_data(Path(Params { uuid }): Path<Params>) -> Json<Vec<(String, u64)>> {
    let timetable = file::get_timetable(uuid).await;

    let data = counter::process_data(timetable);
    
    Json(data)
}

#[derive(Deserialize)]
struct PrefixParams {
    search: String
}

async fn prefix_search(Path(PrefixParams { search }): Path<PrefixParams>) -> Json<Vec<(String, String)>> {
    let names = file::get_all_names().await;

    let mut matches: Vec<(String, String)> = vec![];

    for individual in names {
        let name = individual.0.to_lowercase();
        let checked_search = search.to_lowercase();

        if name.starts_with(&checked_search) {
            matches.push(individual);
        }
    }

    Json(matches)
}

async fn shared_classes(Path(Params { uuid }): Path<Params>) -> Json<Vec<(String, u64)>> {
    Json(counter::shared_classes(uuid).await)
}

async fn countdowns() -> Json<Vec<(String, String)>> {
    let countdowns = countdown::countdowns();

    Json(countdowns)
}

#[shuttle_runtime::main]
pub async fn main() -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_credentials(false);


    println!("ELLO!!");


    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get_all_names", get(get_all_names))
        .route("/get_data/{uuid}", get(get_timetable_data))
        .route("/prefix/{search}", get(prefix_search))
        .route("/shared_classes/{uuid}", get(shared_classes))
        .route("/countdowns", get(countdowns))
        .layer(cors);


    Ok(router.into())
}   


async fn print_files_in_root_dir() -> Vec<String> {
    let mut files = vec![];
    let mut entries = fs::read_dir("./data").await.unwrap();

    while let Some(entry) = entries.next_entry().await.unwrap() {
        let path = entry.path();
        println!("File: {}", path.display());
        files.push(path.display().to_string());
        
    }

    files
}