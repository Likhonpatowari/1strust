fn main() {
    // Rust for loop
    for i in 0..5 {
        // Rust if-else statement
        if i % 2 == 0 {
            // Rust code inside the if block
            println!("Rust Loop Iteration {}: Even", i);
        } else {
            // Rust code inside the else block
            println!("Rust Loop Iteration {}: Odd", i);
        }

        // Call a Rust function inside the loop
        rust_function();
    }

    // Rust code outside the loop
    println!("Rust Code outside the loop.");
}

// Rust function
fn rust_function() {
    println!("This is a Rust function.");
}
