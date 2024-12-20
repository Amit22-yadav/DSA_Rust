// Program to find maximum and minimum in an array

pub fn max_min(){
    let arr = [1,4,5,7,9,22];
    println!("<------------- Program to find max min in an array ------------------>\n");
    println!("Original array: {:?}\n", arr);
    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();
    println!("Min value in array: {}\n", min);
    println!("Max value in array: {}\n", max);
}