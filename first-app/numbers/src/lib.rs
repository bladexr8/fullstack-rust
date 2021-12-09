pub fn say_hello() {
    println!("Hello, world!");
}

// the & symbol creates a slice that represents
// read-only access to vector and array

pub fn print_array() {
    let numbers = [1,2,3,4,5];
    println!("\n***Printing Array***");
    output_sequence(&numbers);
}


pub fn print_vector() {
    let numbers = vec![1,2,3,4,5];
    println!("\n***Printing Vector***");
    // vectors have flexible capacity
    // iterate through - iterator implicitly created
    output_sequence(&numbers);
}


fn output_sequence(numbers: &[u8]) {
    // iterate through slice of
    // arrays or vector
    for n in numbers.iter() {
        println!("{}",n);
    }
}