pub fn countdowns() -> Vec<(String, String)> {
    let cds: Vec<(&str, &str)> = vec![
        ("Last day of school", "2025-04-03 15:20"),
        ("Physics Paper 1", "2025-04-29 13:00"),
        ("SEHS Papers 1 & 3", "2025-04-29 13:00"), 
        ("Physics Paper 2", "2025-04-30 9:00"), 
        ("SEHS Paper 2", "2025-04-30 9:00"), 
        ("Business Papers 1 & 3", "2025-04-30 13:00"), 
        ("Business Paper 2", "2025-05-2 9:00"), 
        ("Computer Science Paper 1", "2025-05-2 13:00"), 
        ("ESS Paper 1", "2025-05-2 13:00"), 
        ("Computer Science Papers 2 & 3", "2025-05-05 9:00"), 
        ("ESS Paper 2", "2025-05-05 9:00"), 
        ("History Papers 1 & 2", "2025-05-05 13:00"), 
        ("History Paper 3", "2025-05-06 9:00"), 
        ("Chinese B Papers 1 & 2", "2025-05-06 13:00"), 
        ("Chinese B Listening", "2025-05-07 9:00"), 
        ("Psychology Paper 1", "2025-05-07 13:00"), 
        ("Psychology Papers 2 & 3", "2025-05-08 9:00"), 
        ("English Paper 1", "2025-05-08 13:00"), 
        ("English Paper 2", "2025-05-09 9:00"), 
        ("Philosophy Paper 1", "2025-05-9 13:00"), 
        ("Geography Paper 1", "2025-05-9 13:00"), 
        ("Geography Papers 2 & 3", "2025-05-12 9:00"), 
        ("Philosophy Papers 2 & 3", "2025-05-12 9:00"), 
        ("Biology Paper 1", "2025-05-12 13:00"), 
        ("Biology Paper 2", "2025-05-13 9:00"), 
        ("Economics Paper 2", "2025-05-13 13:00"), 
        ("Economics Papers 1 & 3", "2025-05-14 9:00"), 
        ("Spanish Papers 1 & 2", "2025-05-14 13:00"), 
        ("Spanish Listening", "2025-05-15 9:00"), 
        ("Math Paper 1", "2025-05-15 13:00"), 
        ("Math Paper 2", "2025-05-16 9:00"), 
        ("Chemistry Paper 1", "2025-05-16 13:00"), 
        ("Chemistry Paper 2", "2025-05-19 9:00"), 
        ("French Papers 1 & 2", "2025-05-20 13:00"), 
        ("French Listening", "2025-05-21 9:00"), 
        ("Math Paper 3", "2025-05-21 13:00"), 
        ("Graduation", "2025-05-28 16:00")
    ];

    let mut parsed: Vec<(String, String)> = vec![];
    for cd in cds {
        parsed.push((cd.0.to_string(), cd.1.to_string()));
    }

    parsed
}