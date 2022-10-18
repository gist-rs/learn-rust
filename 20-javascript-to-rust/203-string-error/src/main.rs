fn example_to_owned() {
    // # to_owned()
    // Own a string from &str by cloning it
    let foo = "foo".to_owned();
    println!("foo = {}", foo);
}

fn example_to_string() {
    // # to_string()
    // Call greet function that requires a String
    greet("World" /*.to_string()*/);
    //❌ expected struct `String`, found `&str`
}

fn example_into() {
    // # into()
    // Convert &str to String
    let foo/*: String*/ = "foo".into();
    //❌ ^^^ consider giving `foo` a type
}

fn example_format() {
    // format!()
    let bar = "bar";
    let foobar = format!("foo {bar}"); //🐥 `foo ${bar}`
    println!("foobar = {foobar}");
}

fn greet(target: String) {
    println!("Hello, {}", target); //🐥 console.log(`Hello, ${target}`);
}

fn example_to_owned_vs_clone() {
    fn foo() -> String {
        "bar".clone() ///⭕️ "bar".to_owned()
        //❌ expected struct `String`, found `&str`
    }
    println!("{:?}", foo());
}

fn main() {
    example_to_owned();
    example_to_string();
    example_into();
    example_format();
    example_to_owned_vs_clone();
}
