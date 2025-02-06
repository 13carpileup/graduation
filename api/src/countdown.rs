pub fn countdowns() -> Vec<(String, String)> {
    let cds: Vec<(&str, &str)> = vec![
        ("Me When", "2025")
    ];

    let mut parsed: Vec<(String, String)> = vec![];
    for cd in cds {
        parsed.push((cd.0.to_string(), cd.1.to_string()));
    }

    parsed
}