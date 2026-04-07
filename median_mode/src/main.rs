use std::collections::HashMap;
fn main() {
    let list = [1,12,4,4,2,3,3,3,3,3,4,512,3,1,3,4,5,56,7,8,9,056,5,7,22,21];
    let mut median_vector = Vec::from(list);
    median_vector.sort();
    let output = median_vector[list.len()/2];
    println!("Median is {output}");

    let mut hash_map = HashMap::new();
    for i in list {
        let count = hash_map.entry(i).or_insert(0);
        *count += 1
    }
    let mode = hash_map.iter().max_by_key(|&(_, v)| v).unwrap().0;
    println!("Mode is {mode}");
}
