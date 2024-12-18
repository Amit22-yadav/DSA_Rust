fn is_palindrome(s: &str) -> bool{
    let rev: String = s.chars().rev().collect();
    s == rev
}

pub fn test_palindrome(){
    print!("<-------------Program to Find String is Palindrome or Not----------->\n\n");
    let data = "madam"; // naman, racecar. 121, 111, etc
    if is_palindrome(&data){
        println!("{}: is palindrome\n", data);
    } else{
        println!("{}: is not a palindrome\n", data);
    }
}