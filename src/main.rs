use std::env::args;
use rand::Rng;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 || args[1].is_empty() || args.len() > 2 {
        println!("Usage: {} <input_string>", args[0]);
        return;
    }
    let input = &args[1];
    let mut output = String::new();
    let mut rng = rand::thread_rng();
    let mut chance: f64;
    let mut threshold = 0.5;
    let mut char: Option<char>;
    for c in input.chars() {
        chance = rng.gen();
        if chance < threshold {
            threshold -= 0.2;
            char = c.to_lowercase().next()
        } else {
            threshold += 0.2;
            char = c.to_uppercase().next()
        };
        if let Some(new_char) = char {
            output.push(new_char);
        }
    }
    println!("{}", output);
}
