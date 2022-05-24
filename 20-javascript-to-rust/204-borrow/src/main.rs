fn shadow() {
    // foo is integer
    let foo = 1; //🐥 var foo = 1;
    println!("{}", foo);

    // foo is string
    let foo = "foo"; //🐥 var foo = "foo";
    println!("{}", foo);
}

fn mutable() {
    // foo is mutable string
    let /*mut*/ foo= "foo"; //🐥 let foo = "foo";
    println!("{}", foo);

    foo = "lol";
    // ❌ ^^^^^^^^^^^ cannot assign twice to immutable variable
    println!("{}", foo);
}


fn borrow() {

}

fn main() {
    shadow();
    mutable();
    borrow();
}
