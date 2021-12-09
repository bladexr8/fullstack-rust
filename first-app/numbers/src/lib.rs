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


pub fn print_vector(limit: u8) {
    let numbers = generate_sequence(limit);
    println!("\n***Printing Vector***");
    // vectors have flexible capacity
    // iterate through - iterator implicitly created
    output_sequence(&numbers);
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    //let mut numbers = Vec::new();
    /*for n in 1..=limit {
        numbers.push(n);
    }*/
    (1..=limit).collect()
    // return value
    // numbers
}

fn output_sequence(numbers: &[u8]) {
    // iterate through slice of
    // arrays or vector
    for n in numbers.iter() {
        println!("{}",n);
    }
}

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1,2,3]);
}