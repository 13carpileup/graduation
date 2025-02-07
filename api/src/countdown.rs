pub fn countdowns() -> Vec<(String, String)> {
    let cds: Vec<(&str, &str)> = vec![
        ("Last day of school", "2025-04-03 15:20"),
        ("Graduation", "2025-05-28 16:00")
    ];

    let mut parsed: Vec<(String, String)> = vec![];
    for cd in cds {
        parsed.push((cd.0.to_string(), cd.1.to_string()));
    }

    parsed
}