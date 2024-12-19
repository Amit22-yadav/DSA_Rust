// Fibonacci series in Rust

fn fibonacci(n: u32)-> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n{
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

pub fn fibonacci_series(){
    let n = 10;
    println!("<------------ Fibonacci series -------------------->\n");
    println!("Fibonacci series of first {} number\n", n);
    for i in 1..n{
        println!("{}", fibonacci(i));
    }
}