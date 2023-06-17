// 소인수 분해

#![allow(unused)]

pub fn solve(n : u64, opt : Option<bool>) {

    let mut vec = vec![];

    next_prime_factor_of(n, &mut vec);

    if let Some(_) = opt {
        println!("{:?}", vec);
    }
}

pub fn next_prime_factor_of(n : u64, factors: &mut Vec<u64> ) -> () {

    for i in 2..n {
        if n % i == 0 {
            factors.push(i);
            next_prime_factor_of(n / i, factors);
            return;
        }
    }

    factors.push(n);
}