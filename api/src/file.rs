use std::fs;
use std::path::Path;

pub async fn get_all_names() -> Vec<super::structs::User> {
    let file_path = "data/names.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let names = contents.split('\n');
    let mut out: Vec<super::structs::User> = vec![];
    
    for name in names {        
        let pair: Vec<&str> = name.split('#').collect();

        // remove last char of name, first char of id
        out.push(
            super::structs::User {
                name: pair[0][0..(pair[0].len()-1)].to_string(), 
                id: pair[1][1..pair[1].len()].to_string().parse::<u64>().unwrap()
            }
        );
    }

    out
}

pub async fn get_timetable(id: u64) -> String {
    let file_path = format!("data/{id}.txt");

    if !Path::new(&file_path).exists() {
        return "Does not exist".to_string();
    }

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}