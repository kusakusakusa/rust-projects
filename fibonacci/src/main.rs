use std::io;

fn main() {
    println!("Enter nth value");
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Faile to read input");
        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error {error}");
                continue;
            }
        };

        if n >= 2 {
            let ans: i32 = fib(n);
            println!("Value of {n} index of Fibonacci sequence is {ans}");
            break;
        } else {
            println!("Enter a value of at least 2");
            continue;
        }
    };
}

fn fib(val: i32) -> i32 {
    if val == 0 {
        0
    } else if val == 1 {
        1
    } else {
        fib(val-1) + fib(val-2)
    }
}
