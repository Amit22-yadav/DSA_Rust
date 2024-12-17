pub mod arrays;
pub mod strings;
use arrays::max_in_array::largest_array;
use strings::string_is_palindrome::test_palindrome;

fn main() {
    largest_array();
    test_palindrome();
}
