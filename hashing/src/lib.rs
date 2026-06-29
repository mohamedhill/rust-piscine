use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}
pub fn median(list: &[i32]) -> i32 {
    let mut new = list.to_vec();
    new.sort();
    if new.len() % 2 == 0 {
        (new[new.len() / 2] + new[new.len() / 2 - 1]) / 2
    } else {
        new[new.len() / 2]
    }
}
pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &n in list {
        *map.entry(n).or_insert(0) += 1;
    }
    *map.iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .0
}