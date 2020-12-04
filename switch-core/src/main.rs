use std::io::{ stdin, Stdin };
use crate::blades::BladesError;

mod blades;

fn main() -> Result<(), blades::BladesError> {
    let mut input_channel = stdin();

    println!("switch switch");
    loop {
        match converse(&mut input_channel) {
            Err(blades::BladesError{ details: "blades!" }) => break,
            _ => continue
        }
    }
    Ok(())
}

fn converse(stdin: &mut Stdin) -> Result<(), blades::BladesError> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let answer = select_answer(buffer.trim_end());
    println!("{}", answer.to_string());
    if answer.trim_end().eq("blades!") {
        return Err(BladesError::new("blades!"));
    }
    Ok(())
}

fn select_answer(input: &str) -> String {
    if input.contains("?") { return "poppin'".to_string(); }
    match input {
        "bye!" => "blades!".to_string(),
        _ => "shmoopie doop".to_string()
    }
}
