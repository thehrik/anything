use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let array = array_input();
    println!("{:?}", counts(&array));
}

fn counts(array: &[i64]) -> HashMap<i64, i64> {
    let mut items: HashMap<i64, i64> = HashMap::new();

    for &num in array {
        // Try to find the item in the vector
        items.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }

    items
}

fn array_input() -> Vec<i64> {
    let mut s: Vec<i64> = Vec::new();

    loop {
        print!("Enter Number choice [q to quit]: ");
        io::stdout().flush().unwrap();

        let mut val = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read line");

        let input = val.trim();

        if input == "q" {
            return s;
        }

        if input.is_empty() {
            println!("No inputs, try typing a number");
            continue;
        }

        match input.parse() {
            Ok(n) => s.push(n),
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        }
    }
}
