use std::collections::HashMap;

pub fn get_hash<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

pub fn get_value_from_ident(x: &String, store: HashMap<&str, String>) -> String {
    if x.chars()
        .map(|f| f.is_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        x.chars()
            .map(|f| {
                if f.is_alphabetic() {
                    store.get(&*f.to_string()).unwrap().to_string()
                } else {
                    f.to_string()
                }
            })
            .collect()
    } else {
        x.to_string()
    }
}
