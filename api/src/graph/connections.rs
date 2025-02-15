// a connection between two people is described as an integer
// between 0 - 10

// the map of all connections is the connections between all people

use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::Column;
use sqlx::ValueRef;
use sqlx::Row;
use std::collections::HashMap;

use crate::structs::User;
use super::get_all_names;
use super::add_shared_classes;
use super::write_to_file;

// helpers 
pub fn row_to_json(row: &PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = match value.is_null() {
            true => "NULL".to_string(),
            false => {
                match value.as_str() {
                    Ok(e) => e.to_string(),
                    Err(v) => {println!("{v}");"plu".to_string()},
                }
            },
        };
        result.insert(
            col.name().to_string(),
            value,  
        );
    }

    result
}

// connectors
pub async fn init_database() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/grad")
        .await?;

    sqlx::query(
        "DROP TABLE Connections;"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Connections (
            connection VARCHAR(255) PRIMARY KEY,
            weight VARCHAR(255) NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    let students = get_all_names().await;

    let mut counter = 0;
    let total = students.len();

    for s1 in &students {
        counter += 1;
        println!("{counter}/{total} {a1}", a1 = s1.1);

        let mut resp = add_shared_classes(s1.1.parse::<u64>().unwrap(), false).await.1;
        resp.sort_by(|a, b| {
            a.1.cmp(&b.1)  
        });

        let rev: Vec<&(User, u64)> = resp.iter().rev().collect();

        println!("{fst}", fst = rev[0].1);

        for i in 0..2 {
            let s2 = rev[i];

            sqlx::query(
                &format!(
                "INSERT INTO Connections VALUES ('{a1}-{a2}', '{val}') ON CONFLICT (connection) DO UPDATE SET weight = EXCLUDED.weight;",
                a1 = s1.1, a2 = s2.0.id, val = s2.1
                )
            )
            .execute(&pool)
            .await?;
        }
    }
    
    Ok(())
}

pub async fn get_connections() -> Result<Vec<((String, String), u64)>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/grad")
        .await?;

    // match init_database().await {
    //     Ok(_e) => println!("Successfully initiated"),
    //     Err(v) => {
    //         println!("Initiation Failed");
    //         println!("{v}");
    //         return Err(v);
    //     }
    // };

    let resp = sqlx::query(
        &format!(
        "SELECT * FROM Connections"
        )
    )
    .fetch_all(&pool)
    .await?;

    let mut out = vec![];

    println!("Number of entries: {len}", len = resp.len());

    for row in resp {
        let formatted = row_to_json(&row);
        //println!("preformatted: {connection} has weight {weight}", connection = formatted["connection"].clone(), weight = formatted["weight"].clone());

        let primary_key = formatted["connection"].clone();
        let ids: Vec<&str> = primary_key.split('-').collect();

        let new_entry = ((ids[0].to_string(), ids[1].to_string()), formatted["weight"].clone().parse::<u64>().unwrap());
        
        out.push(new_entry);
    }

    Ok(out)
}

pub async fn update_connection(id1: String, id2: String, delta: i64) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:postgres@localhost/grad")
    .await?;

    write_to_file(format!("Updating {id1} and {id2} by {delta}")).await;

    let resps = sqlx::query(&format!("SELECT * FROM Connections WHERE connection = '{id1}-{id2}'")).fetch_all(&pool).await?;

    let original_weight = &row_to_json(&resps[0])["weight"].parse::<i64>().unwrap();
    let weight: i64 = *original_weight + delta;

    sqlx::query(
        &format!(
        "UPDATE Connections SET weight = {weight} WHERE connection = '{id1}-{id2}'"
        )
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        &format!(
        "UPDATE Connections SET weight = {weight} WHERE connection = '{id2}-{id1}'"
        )
    )
    .execute(&pool)
    .await?;

    Ok(())
}
