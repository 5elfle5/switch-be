pub fn select_answer(input: &str) -> String {
    if input.contains("?") { return "poppin'".to_string(); }
    match input {
        "bye!" => "blades!".to_string(),
        _ => "shmoopie doop".to_string()
    }
}
