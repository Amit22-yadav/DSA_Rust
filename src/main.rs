pub mod arrays;
pub mod strings;
use arrays::max_in_array::largest_array;
use strings::string_is_palindrome::test_palindrome;
use strings::longest_string::find_long_string;
use arrays::Fibonacci::fibonacci_series;
use arrays::reverse_array::rev_arr;
use arrays::max_min_array::max_min;
use arrays::duplicate_in_arr::duplicates_numbers;
use arrays::sort_arr::sort_array;

fn main() {
    largest_array();
    test_palindrome();
    find_long_string();
    fibonacci_series();
    rev_arr();
    max_min();
    duplicates_numbers();
    sort_array();
}
