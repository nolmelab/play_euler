//! 1000미만의 3 또는 5의 배수의 개수를 구하라
//! 
//! 3의 배수의 개수와 5의 배수의 개수에서 3과 5의 공배수의 개수를 빼면 된다. 
//! 
//! 다른 방법은 없는가? 
//! 
//! 3배 배수의 집합을 만들고, 5의 배수의 집합을 만든 다음 두 집합의 
//! 합집합을 만들면 된다. 
//! 
//! 
//! 

#![allow(unused)]

use std::collections::HashSet;
use crate::util::sort;

pub fn solve(max: u64) {

    let mut m_3 ;
    let mut m_5 ;

    let mut r = HashSet::<u64>::new();

    for i in 1..500 {

        m_3 = 3 * i;

        if m_3 >= max {
            break;
        }

        r.insert(m_3);
    }

    for i in 1..500 {

        m_5 = 5 * i;

        if m_5 >= max {
            break;
        }

        r.insert(m_5);
    }

    let vec = sort(&r);

    assert!(vec.len() == r.len());

    let mut sum = 0;
    for v in &vec {
        sum += v;
    }

    println!("count: {}", vec.len());
    println!("sum: {}", sum);
    println!("elements: {:?}",vec);

    // how to prove the above is correct? 
    // 
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn learn_hashset() {

        let mut vs = HashSet::<u64>::new();

        assert!(vs.insert(3));
        assert!(vs.contains(&3));
        assert!(vs.remove(&3));
        assert!(vs.insert(3));
        assert!(vs.insert(4));

        for v in vs.iter() {
            println!("{v}");
        }
    }
}