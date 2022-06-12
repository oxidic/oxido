use std::collections::HashMap;

pub fn get_hash<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

pub fn is_numeric(line: &str) -> bool {
    !line.chars().map(|c| c.is_numeric()).collect::<Vec<bool>>().contains(&false)
}