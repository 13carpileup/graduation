use std::fs::OpenOptions;
use std::io::Write;

pub async fn write_to_file(content: String) {
    let mut data_file = OpenOptions::new()
    .append(true)
    .open("log.txt")
    .expect("cannot open file");

    data_file
        .write(format!("{content}\n").as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}