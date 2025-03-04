use std::io;

fn main() {
    println!("Enter 1st Matrix");
    let matrix1 = matrix_input(matrix_size());
    println!("Enter 2nd Matrix");
    let matrix2 = matrix_input(matrix_size());

    if matrix1.len() == 0 && matrix1[0].len() == 0 {
        if matrix2.len() == 0 && matrix2[0].len() == 0 {
            println!("Invalid Matrices");
            return;
        }
    }

    if matrix2.len() != matrix1[0].len(){
        println!("matrix1 and matrix2 cannot be multiplied");
        return;
    }

    println!("Your Multiplied Matrix is");
    println!("{:?}", matrix_multiplier(&matrix1,&matrix2));
}

fn input() -> i32{
    let output;

    loop{
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(num) => {
                        output = num;
                        break;
                    }
                    Err(_) => {
                        println!("Plz enter a number");
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("Plz enter a number");
                continue;
            }
        }
    }
    output
}

fn matrix_size() -> (i32, i32) {
    println!("Please enter number of rows");
    let rows = input();
    println!("Please enter number of columns");
    let cols = input();

    (rows, cols)
}

fn matrix_input(size: (i32, i32)) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut i = 0;
    while i < size.0{
        let mut row: Vec<i32> = Vec::new();
        let mut j = 0;
        while j < size.1 {
            let mut input = String::new();

            println!("value of {} x {}: ", j+1, i+1);

            io::stdin().read_line(&mut input).unwrap();

            let trimmed = input.trim();
            if trimmed == "end" {
                break; // End input for the current row
            }

            match trimmed.parse::<i32>() {
                Ok(num) => row.push(num), // Add number to the current row
                Err(_) => {
                    println!("Invalid input, please enter a number or 'end'!");
                    continue;
                },
            }
            j += 1
        }
        matrix.push(row);
        i += 1;
    }
    matrix
}

fn matrix_multiplier(matrix1: &Vec<Vec<i32>>, matrix2: &Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut multiplied_matrix: Vec<Vec<i32>> = Vec::new();

    for i in 0..matrix1.len() {
        let row: Vec<i32> = Vec::new();
        multiplied_matrix.push(row);
        for k in 0..matrix2[0].len() {
            let mut sum = 0;
            for l in 0..matrix1[0].len() {
                sum += matrix1[i][l]*matrix2[l][k];
            }
            multiplied_matrix[i].push(sum);
        }
    }
    multiplied_matrix
}