use std::fs::OpenOptions;
use std::io::Write;
use chrono;

pub async fn write_to_file(content: String) {
    let mut data_file = OpenOptions::new()
    .append(true)
    .open("log.txt")
    .expect("cannot open file");

    let date = chrono::offset::Local::now();

    data_file
        .write(format!("{date}: {content}\n").as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}