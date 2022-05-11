// Main function
fn main() {
    // Call greet function
    greet("World" /*.to_string()*/); // 💡 Expected error, What is the problem?
}

// Greet function
fn greet(target: String) {
    // console.log(`Hello, ${target}`);
    println!("Hello, {}", target);
}

// Test function
#[test]
fn test() {
    // Call main function
    main();
}
