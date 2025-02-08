pub fn countdowns() -> Vec<(String, String)> {
    let cds: Vec<(&str, &str)> = vec![
        ("Last day of school", "2025-04-03 15:20"),
        ("Physics Paper 1", "2025-04-29 12:30"),
        ("SEHS Papers 1 & 3", "2025-04-29 12:30"), 
        ("Physics Paper 2", "2025-04-30 8:30"), 
        ("SEHS Paper 2", "2025-04-30 8:30"), 
        ("Business Papers 1 & 3", "2025-04-30 12:30"), 
        ("Business Paper 2", "2025-05-2 8:30"), 
        ("Computer Science Paper 1", "2025-05-2 12:30"), 
        ("ESS Paper 1", "2025-05-2 12:30"), 
        ("Computer Science Papers 2 & 3", "2025-05-05 8:30"), 
        ("ESS Paper 2", "2025-05-05 8:30"), 
        ("History Papers 1 & 2", "2025-05-05 12:30"), 
        ("History Paper 3", "2025-05-06 8:30"), 
        ("Chinese B Papers 1 & 2", "2025-05-06 12:30"), 
        ("Chinese B Listening", "2025-05-07 8:30"), 
        ("Psychology Paper 1", "2025-05-07 12:30"), 
        ("Psychology Papers 2 & 3", "2025-05-08 8:30"), 
        ("English Paper 1", "2025-05-08 12:30"), 
        ("English Paper 2", "2025-05-09 8:30"), 
        ("Philosophy Paper 1", "2025-05-9 12:30"), 
        ("Geography Paper 1", "2025-05-9 12:30"), 
        ("Geography Papers 2 & 3", "2025-05-12 8:30"), 
        ("Philosophy Papers 2 & 3", "2025-05-12 8:30"), 
        ("Biology Paper 1", "2025-05-12 12:30"), 
        ("Biology Paper 2", "2025-05-13 8:30"), 
        ("Economics Paper 2", "2025-05-13 12:30"), 
        ("Economics Papers 1 & 3", "2025-05-14 8:30"), 
        ("Spanish Papers 1 & 2", "2025-05-14 12:30"), 
        ("Spanish Listening", "2025-05-15 8:30"), 
        ("Math Paper 1", "2025-05-15 12:30"), 
        ("Math Paper 2", "2025-05-16 8:30"), 
        ("Chemistry Paper 1", "2025-05-16 12:30"), 
        ("Chemistry Paper 2", "2025-05-19 8:30"), 
        ("French Papers 1 & 2", "2025-05-20 12:30"), 
        ("French Listening", "2025-05-21 8:30"), 
        ("Math Paper 3", "2025-05-21 12:30"), 
        ("Graduation", "2025-05-28 16:00")
    ];

    let mut parsed: Vec<(String, String)> = vec![];
    for cd in cds {
        parsed.push((cd.0.to_string(), cd.1.to_string()));
    }

    parsed
}