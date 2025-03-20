pub fn countdowns() -> Vec<(String, String)> {
    let cds: Vec<(&str, &str)> = vec![
        ("Orals", "2025-03-11 8:00"),
        ("Last day of school", "2025-04-03 15:20"),
        ("Physics Paper 1", "2025-04-29 13:30"),
        ("SEHS Papers 1 & 3", "2025-04-29 13:30"), 
        ("Physics Paper 2", "2025-04-30 9:30"), 
        ("SEHS Paper 2", "2025-04-30 9:30"), 
        ("Business Papers 1 & 3", "2025-04-30 13:30"), 
        ("Business Paper 2", "2025-05-2 9:30"), 
        ("Computer Science Paper 1", "2025-05-2 13:30"), 
        ("ESS Paper 1", "2025-05-2 13:30"), 
        ("Computer Science Papers 2 & 3", "2025-05-05 9:30"), 
        ("ESS Paper 2", "2025-05-05 9:30"), 
        ("History Papers 1 & 2", "2025-05-05 13:30"), 
        ("History Paper 3", "2025-05-06 9:30"), 
        ("Chinese B Papers 1 & 2", "2025-05-06 13:30"), 
        ("Chinese B Listening", "2025-05-07 9:30"), 
        ("Psychology Paper 1", "2025-05-07 13:30"), 
        ("Psychology Papers 2 & 3", "2025-05-08 9:30"), 
        ("English Paper 1", "2025-05-08 13:30"), 
        ("English Paper 2", "2025-05-09 9:30"), 
        ("Philosophy Paper 1", "2025-05-9 13:30"), 
        ("Geography Paper 1", "2025-05-9 13:30"), 
        ("Geography Papers 2 & 3", "2025-05-12 9:30"), 
        ("Philosophy Papers 2 & 3", "2025-05-12 9:30"), 
        ("Biology Paper 1", "2025-05-12 13:30"), 
        ("Biology Paper 2", "2025-05-13 9:30"), 
        ("Economics Paper 2", "2025-05-13 13:30"), 
        ("Economics Papers 1 & 3", "2025-05-14 9:30"), 
        ("Spanish Papers 1 & 2", "2025-05-14 13:30"), 
        ("Spanish Listening", "2025-05-15 9:30"), 
        ("Math Paper 1", "2025-05-15 13:30"), 
        ("Math Paper 2", "2025-05-16 9:30"), 
        ("Chemistry Paper 1", "2025-05-16 13:30"), 
        ("Chemistry Paper 2", "2025-05-19 9:30"), 
        ("French Papers 1 & 2", "2025-05-20 13:30"), 
        ("French Listening", "2025-05-21 9:30"), 
        ("Math Paper 3", "2025-05-21 13:30"), 
        ("Graduation", "2025-05-28 16:30")
    ];

    let mut parsed: Vec<(String, String)> = vec![];
    for cd in cds {
        parsed.push((cd.0.to_string(), cd.1.to_string()));
    }

    parsed
}