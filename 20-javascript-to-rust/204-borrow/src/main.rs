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


fn string_convert() {
    let foo_str = "foo";
    let mut foo_string: String = foo_str.to_string();
    println!("foo_str:{}", foo_str);
    println!("foo_string:{}", foo_string);

    foo_string = "baz".to_string();
    println!("foo_string:{}", foo_string);

    foo_string = "baz baz".to_owned();
    println!("foo_string:{}", foo_string);

    foo_string = "baz baz baz".into();
    println!("foo_string:{}", foo_string);
}

fn borrow() {
    // immutable str
    bar_str("foo");

    // immutable string
    bar_string("foo".to_string());

    // mutable str
    let mut foo_str = "foo";
    bar_str_mut(&mut foo_str);
    println!("3️⃣ target_str:{}", foo_str);

    // mutable string
    let mut foo_string = "foo".to_owned();
    bar_string_mut( &mut foo_string);
    println!("3️⃣ target_string:{}", foo_string);
}

fn bar_str(target_str: &str) {
    println!("1️⃣ target_str:{}", target_str);
}

fn bar_string(target_string: String) {
    println!("1️⃣ target_string:{}", target_string);
}

fn bar_str_mut(target_str: &mut &'static str) {
    println!("1️⃣ target_str:{}", target_str);
    *target_str = "baz";
    println!("2️⃣ target_str:{}", target_str);
}

fn bar_string_mut(target_string: &mut String) {
    println!("1️⃣ target_string:{}", target_string);
    *target_string = "baz".to_owned();
    println!("2️⃣ target_string:{}", target_string);
}

fn main() {
    shadow();
    mutable();
    string_convert();
    borrow();
}
