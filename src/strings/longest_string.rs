// Longest String in an array

fn long_string<'a>(strings: &'a [&'a str]) -> &'a str{
    let mut longest = "";
    for &s in strings{
        if s.len() > longest.len(){
            longest = s;
        }
    }
    longest
}

pub fn find_long_string(){
    println!("<------------ Longest String in the Array-------------------->\n");
    let arr = ["Apple", "Car", "Laptop", "Cycle"];
    let output = long_string(&arr);
    println!("Longest String in the Array is: {}\n\n", output);
}