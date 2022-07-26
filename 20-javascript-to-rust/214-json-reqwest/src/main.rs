use serde_json::Value;

fn main() {
    let foo_str = r#"[
        {"id": "sol.usdc"},
        {"id": "sol.sol"}
    ]"#;

    let foo_json: Vec<Value> = serde_json::from_str::<Vec<Value>>(foo_str).unwrap();
    println!("foo_json = {:#?}", foo_json);

    // filter then map
    let filtered_foo_json = foo_json
        .iter()
        .filter(|v| v["foo"] == "sol.usdc")
        .map(|v| v.to_owned())
        .collect::<Vec<Value>>();

    println!("filter_and_map_foo_json = {:#?}", filtered_foo_json);

    // filter_map
    let filtered_foo_json = foo_json
        .iter()
        .filter_map(|v| {
            if v["foo"] == "sol.usdc" {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<Value>>();

    println!("filter_map_foo_json = {:#?}", filtered_foo_json);

    // filter then map
    let filtered_foo_value_json = foo_json
        .iter()
        .filter(|v| v["foo"] == "sol.usdc")
        .map(|v| v["foo"].as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    println!("filtered_foo_value_json = {:#?}", filtered_foo_value_json);
}
