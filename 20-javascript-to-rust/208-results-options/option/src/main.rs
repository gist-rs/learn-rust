fn main() {
    // Ignore None
    let foo = vec![Some(1), Some(2), None];
    let filter_foo = foo.iter().filter(|e| e.is_some()).collect::<Vec<_>>();
    let filter_map_foo = foo.iter().filter_map(|e| e.as_ref()).collect::<Vec<_>>();
    let flat_map_foo = foo.iter().flat_map(|e| e).collect::<Vec<_>>();
    let flatten_foo = foo.iter().flatten().collect::<Vec<_>>();

    println!("foo: {:?}", foo);
    println!("filter_foo: {:?}", filter_foo);
    println!("filter_map_foo: {:?}", filter_map_foo);
    println!("flat_map_foo: {:?}", flat_map_foo);
    println!("flatten_foo: {:?}", flatten_foo);

    // Collect none
    println!("----------------------------------------------");
    let foo = vec![Some(1), Some(2), None];

    let filter_map_err_foo = foo
        .iter()
        .filter_map(|r| match r {
            Some(r) => Some(r),
            None => None,
        })
        .collect::<Vec<_>>();

    println!("filter_map_err_foo: {:?}", filter_map_err_foo);
}
