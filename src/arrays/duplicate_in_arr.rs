use std::collections::HashSet;
// find duplicates in an array
fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    print!("<-------------Program to Find duplicate element in an Array----------->\n\n");
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for &item in arr {
        if !seen.insert(item) {
            duplicates.push(item);
        }
    }

    duplicates
}

pub fn duplicates_numbers() {
    let arr = [1, 2, 3, 4, 5, 2, 3, 6, 7, 3];
    let duplicates = find_duplicates(&arr);
    println!("Original array: {:?}\n", arr);
    println!("Duplicates in array are:- {:?}\n\n", duplicates);
}