use std::io::{self, Write};

struct Items {
    value: i64,
    count: i64,
}

fn main() {
    let array = array_input();
    let counts = counts(&array);
    for item in counts {
        println!("{}: {}", item.value, item.count);
    }
}

fn counts(array: &Vec<i64>) -> Vec<Items> {
    let mut items: Vec<Items> = Vec::new();

    for &num in array {
        // Try to find the item in the vector
        if let Some(item) = items.iter_mut().find(|item| item.value == num) {
            item.count += 1;
        } else {
            items.push(Items { value: num, count: 1 });
        }
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
