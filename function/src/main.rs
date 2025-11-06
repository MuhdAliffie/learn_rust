use std::io;

fn main() {
    let mut program_counter: i32 = 0;

    'count_program: loop {
        println!("This is a simple calculator.");
        let a: i32 = request_num();
        let b: i32 = request_num();
        let result = operand_handler(a, b);
        println!("The result is: {}", result);

        program_counter += 1;
        if program_counter >= 3 {
            println!("Program has run 3 times. Exiting.");
            break 'count_program;
        }
    }
}

fn request_num() -> i32 {
    let mut input_num = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");
    let processed_num: i32 = input_num
        .trim()
        .parse()
        .expect("Please enter a valid number");

    processed_num
}

fn operand_handler(a: i32, b: i32) -> f64 {
    println!(
        "What you want to do? Input 1, 2, 3, or 4:\n
    1. Add\n
    2. Subtract\n
    3. Multiply\n
    4. Divide"
    );

    let mut operand = String::new();
    io::stdin()
        .read_line(&mut operand)
        .expect("Failed to read line");

    let operand: i32 = operand.trim().parse().expect("Invalid input");

    let num: f64 = match operand {
        1 => (a + b) as f64, // Returns floating point
        2 => (a - b) as f64,
        3 => (a * b) as f64,
        4 => a as f64 / b as f64, // This gives true division, not integer division
        _ => {
            eprintln!("Invalid operand");
            return 0.0;
        }
    };

    num
}
