fn main() {
        // addition
    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The quotient is: {}", quotient);
    println!("The truncated division is: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The first value of the tuple is: {}", tup.0);
    println!("The second value of the tuple is: {}", tup.1);
    println!("The third value of the tuple is: {}", tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The first element of the array is: {}", first);

    let b = [3; 5];
    println!("The array b is: {:?}", b);

}
