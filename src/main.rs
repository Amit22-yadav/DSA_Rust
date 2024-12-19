pub mod arrays;
pub mod strings;
use arrays::max_in_array::largest_array;
use strings::string_is_palindrome::test_palindrome;
use strings::longest_string::find_long_string;
use arrays::Fibonacci::fibonacci_series;

fn main() {
    largest_array();
    test_palindrome();
    find_long_string();
    fibonacci_series();
}
