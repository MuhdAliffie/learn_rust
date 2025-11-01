use std::io;

fn main() {
    let A = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Invalid index");

    let element = A[index];

    println!("The value of the element at index {} is: {}", index, element);
}
