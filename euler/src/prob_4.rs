#![allow(unused)]

// largest palindrome product
pub fn solve(start : u64, end : u64, opt : Option<bool>) {

    let mut vec = vec![];

    for v in start..end {
        for w in start..end {
            let r = v * w;

            if is_palindrome(r) {
                vec.push(r);
            }
        }
    }

    if let Some(_) = opt {
        vec.sort();
        println!("{:?}", vec);
    }
}

fn is_palindrome(n : u64) -> bool {

    let fs = format!("{}", n);
    let rs: String = fs.chars().rev().collect();

    fs == rs
}