use std::collections::{HashMap, HashSet};
use chrono::{NaiveDate, Utc};

use crate::structs::User;

fn check_date(uuid: &str) -> bool {
    let day = uuid[6..8].to_string().parse::<u32>().unwrap(); 
    let month = uuid[4..6].to_string().parse::<u32>().unwrap(); 
    let hours = uuid[10..11].to_string().parse::<u32>().unwrap();
    let minutes = uuid[11..13].to_string().parse::<u32>().unwrap();

    let year = 2025;
    let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    
    let naive_date_time = naive_date.and_hms_opt(hours, minutes, 0).unwrap();

    let current_utc_time = Utc::now().naive_utc();

    current_utc_time < naive_date_time
}

pub fn process_data(timetable: String) -> Vec<(String, u64)> {
    let lines = timetable.split("\r\n");

    let mut counter: HashMap<String, u64> = HashMap::new();

    let mut current_class = "";

    for line in lines {
        if line.starts_with("SUMMARY") {
            let class_code = line.split(':').collect::<Vec<&str>>()[1];
            current_class = class_code;

            if !counter.contains_key(current_class) {
                counter.insert(current_class.to_string(), 0);
            }
        }

        if line.starts_with("DTSTART") {
            let uuid = line.split(':').collect::<Vec<&str>>()[1];

            if check_date(uuid) {
                let entry = counter.entry(current_class.to_string()).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut out: Vec<(String, u64)> = vec![];

    for (key, value) in counter {
        out.push((key, value));
    }

    return out;
}

pub async fn add_shared_classes(uuid: u64, log: bool, key: String) -> (Vec<(String, u64)>, Vec<(User, u64)>) {
    let mut out: Vec<(User, u64)> = vec![];

    let raw_data = super::file::get_timetable(uuid).await;
    let processed_data = process_data(raw_data);

    let all_students = super::file::get_all_names().await;
    for student in all_students {
        let new_uuid = student.id;
        if new_uuid == uuid {
            if log {
                let _ = super::log::write_to_file(format!("{key} fetched {name}", key = key, name = student.name)).await;
            }
            continue;
        }

        let mut total_classes = 0;

        let rdata = super::file::get_timetable(new_uuid).await;
        let pdata = process_data(rdata);

        for e1 in &pdata {
            for e2 in &processed_data {
                if e1.0 == e2.0 {
                    total_classes += e1.1;
                }
            }
        }

        out.push((User {name: student.name, id: new_uuid}, total_classes));
    }
    
    (processed_data, out)
}

/// returns vector of classes, sorted alphabetically
pub async fn get_classes(id: u64) -> Vec<String> {
    let raw_data = super::file::get_timetable(id).await;
    let lines = raw_data.split("\r\n");
    let mut classes: HashSet<String> = HashSet::new();

    for line in lines {
        if line.starts_with("SUMMARY:") {
            let class_code = &line.split(':').collect::<Vec<&str>>()[1][2..5];

            classes.insert(class_code.to_string());

            //println!("{class_code}");
        }
    }

    let mut v: Vec<String> = classes.into_iter().collect();
    v.sort();

    v
}