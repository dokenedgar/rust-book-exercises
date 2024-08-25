fn main() {
    let sequence: u32 = 15;
    let value = fibonacci(sequence);
    println!("The F({sequence}) value of Fib is {value}");
}

fn fibonacci(x: u32) -> u32 {
    let mut result: u32 = 0;
    if x == 0 {
        result = 0;
    } else if x == 1 || x == 2 {
        result = 1;
    } else {
        result = fibonacci(x - 1) + fibonacci(x - 2);
    }
    return result;
}
