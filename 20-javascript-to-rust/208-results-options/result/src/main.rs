fn main() {
    // Ignore errors
    let foo = vec![Ok(1), Ok(2), Err(3)];
    let filter_foo = foo.iter().filter(|e| e.is_ok()).collect::<Vec<_>>();
    let filter_map_foo = foo.iter().filter_map(|e| e.ok()).collect::<Vec<_>>();
    let flat_map_foo = foo.iter().flat_map(|e| e).collect::<Vec<_>>();
    let flatten_foo = foo.iter().flatten().collect::<Vec<_>>();

    println!("foo: {:?}", foo);
    println!("filter_foo: {:?}", filter_foo);
    println!("filter_map_foo: {:?}", filter_map_foo);
    println!("flat_map_foo: {:?}", flat_map_foo);
    println!("flatten_foo: {:?}", flatten_foo);

    // Collect errors
    println!("----------------------------------------------");
    let foo = vec![Ok(1), Ok(2), Err("This is an error!")];

    let mut errors = vec![];
    let filter_map_err_foo = foo
        .iter()
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect::<Vec<_>>();

    println!("filter_map_err_foo: {:?}", filter_map_err_foo);
    println!("errors: {:?}", errors);
}
