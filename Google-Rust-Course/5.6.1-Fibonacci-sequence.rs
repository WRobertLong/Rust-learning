/*
The Fibonacci sequence begins with [0, 1]. For n > 1, the next number is the sum of the previous two.

 This code implements the recursive approach. The itarative approach will be more efficient   

*/

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}