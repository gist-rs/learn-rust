fn shadow() {
    // foo is integer
    // var foo = 1;
    let foo = 1;
    println!("{}", foo);

    // foo is string
    // var foo = "foo";
    let foo = "foo";
    println!("{}", foo);

    // foo is string
    let /*mut*/ bar= "bar";
    // bar = "lol";
    // ❌ ^^^^^^^^^^^ cannot assign twice to immutable variable
    println!("{}", bar);
}

fn borrow() {}

fn main() {
    shadow();
    borrow();
}
