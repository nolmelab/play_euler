#![allow(unused)]

pub fn solve(n : u64, opt : bool) {
    let mut primes = vec![2];

    for i in 3.. {
        let mut is_prime = true;
        for p in &primes {
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i);
        }

        if primes.len() >= n as usize {
            break;
        }
    }

    if opt {
        println!("{:?}", primes);
    }
}

pub fn is_prime(n : u64) -> bool {

    for i in 2..n {
        if i % n == 0 {
            return false;
        }
    }

    true
}


