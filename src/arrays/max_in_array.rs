pub fn largest_array(){
    print!("<-------------Program to Find Largest element in an Array----------->\n");
    let a = [5,7,9,2,0,10];
    print!("Length of Given Array is :- {} \n ",a.len());
    let mut max:i32 = 0;
    let mut i:usize = 0;
    max = a[i];
    while i < a.len(){
        if max < a[i]{
            max = a[i];
        }
        i += 1;
    }
    println!("Largest Element in the Array is :- {}\n ",max);
}