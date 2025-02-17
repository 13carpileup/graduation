use axum::{
    extract::Path,
    routing::get, 
    Json, 
};
use shuttle_axum::axum::Router; // Import Router from shuttle_axum::axum
use serde::Deserialize;
use structs::User;
use tower_http::cors::{Any, CorsLayer}; 
use http::Method;
use std::env;

mod file;
mod counter;
mod countdown;
mod log;
mod graph;
pub mod structs;

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn get_all_names() -> Json<Vec<User>> {
    let res = file::get_all_names().await;
    
    Json(res)
}

#[derive(Deserialize)]
struct Params {
    uuid: u64
}

#[derive(Deserialize)]
struct TrackingParams {
    uuid: u64,
    key: String
}

async fn get_timetable_data(Path(TrackingParams { uuid, key }): Path<TrackingParams>) -> Json<(Vec<(String, u64)>, Vec<(User, u64)>)> {
    let data = counter::add_shared_classes(uuid, true, key).await;
    
    Json(data)
}

#[derive(Deserialize)]
struct PrefixParams {
    search: String
}

async fn prefix_search(Path(PrefixParams { search }): Path<PrefixParams>) -> Json<Vec<User>> {
    let names = file::get_all_names().await;

    let mut matches: Vec<User> = vec![];

    for individual in names {
        let name = individual.name.to_lowercase();
        let checked_search = search.to_lowercase();

        if name.starts_with(&checked_search) {
            matches.push(individual);
        }
    }

    Json(matches)
}

async fn countdowns() -> Json<Vec<(String, String)>> {
    let countdowns = countdown::countdowns();

    Json(countdowns)
}

async fn get_connections() -> Json<(Vec<Vec<((String, String), u64)>>, Vec<(User, (i32, i32, i32))>)> {
    let students = file::get_all_names().await;

    let conn1 = graph::connections::get_classes_connections().await.unwrap();
    let conn2 = graph::connections::get_subjects_connections(&students).await;
    
    let colours = graph::colour::get_colourings(students).await;
    
    Json((vec![conn1, conn2], colours))
}

#[derive(Deserialize)]
struct MultiParams {
    uuid1: String,
    uuid2: String,
    delta: i64,
}

async fn update_connections(Path(MultiParams { uuid1, uuid2, delta }): Path<MultiParams>) -> Json<String> {
    println!("{uuid1} and {uuid2}");

    if delta != 1 && delta != -1 {
        log::write_to_file(format!("Direct delta writing attempt for users {uuid1} and {uuid2}")).await;
        return Json("Invalid delta".to_string());
    }
    
    let resp = graph::connections::update_connection(uuid1, uuid2, delta).await;

    match resp {
        Ok(_e) => return  Json("Success".to_string()),
        Err(v) => return Json(v.to_string())
    };
}

async fn get_classes(Path(Params { uuid }): Path<Params>) -> Json<Vec<String>> {
    let resp = counter::get_classes(uuid).await;

    Json(resp)
}

// async fn max_connections() -> Json<u64> {
//     let names = file::get_all_names().await;
//     let mut mx = 0;
//     let mut mxPair: String = "a".to_string();
//     println!("working on max...");

//     for name in names {
//         let resp = counter::add_shared_classes(name.1.parse::<u64>().unwrap(), false).await.1;

//         for pair in resp {
//             let number = pair.1;

//             if mx < number {
//                 mx = number;
//                 mxPair = pair.0.0.clone();
//             }
//         }
//     }

//     println!("max: {mx} {mxPair}");

//     Json(mx)
// }

#[shuttle_runtime::main]
pub async fn main() -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_credentials(false);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get_all_names", get(get_all_names))
        .route("/get_data/{uuid}/{key}", get(get_timetable_data))
        .route("/prefix/{search}", get(prefix_search))
        .route("/countdowns", get(countdowns))
        .route("/get_connections", get(get_connections))
        .route("/update_connections/{uuid1}/{uuid2}/{delta}", get(update_connections))
        .route("/get_classes/{uuid}", get(get_classes))
        .layer(cors);

    //max_connection
//	match connections::init_database().await {
//		Ok(_e) => println!("Successfully initted"),
//		Err(v) => println!("{v}")
//	};

    counter::get_classes(9668).await;

    Ok(router.into())
}   
