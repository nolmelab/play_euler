#![allow(unused)]

pub fn solve(max : u64, debug: Option<bool>) {
    let mut sum = 0_u64;

    let mut n1 = 1;
    let mut n2 = 2;

    loop {
        assert!(n2 > n1);

        if n1 > max {
            break;
        }

        if is_even(n1) {
            sum += n1;
        }

        if is_even(n2) {
            sum += n2;
        }

        if let Some(_) = debug {
            println!("{}", n1);
            println!("{}", n2);
        }

        n1 = fib(n1, n2);
        n2 = fib(n2, n1);
    }

    println!("sum: {}", sum);
}

fn fib(n1 : u64, n2 : u64) -> u64 {
    n1 + n2
}

fn is_even(n : u64 ) -> bool {
    n % 2 == 0
}