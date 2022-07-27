// Convert kv from tuple to HashMap
let map: HashMap<_, _> = HashMap::from_iter(p
    .iter()
    .filter(|(k, _)| !k.is_empty())
);

//or
p.iter().filter(|(k, _)| !k.is_empty()).collect::<HashMap<&str,&str>>()