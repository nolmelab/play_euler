use std::collections::HashSet;

pub fn sort<T>(set : &HashSet<T>) -> Vec<T> 
where T : Ord + Copy
{
    let mut vec = vec![];

    for v in set.iter() {
        let rc = vec.binary_search(v);
        match rc {
            Ok(ix) => {
                vec.insert(ix, *v);
            }
            Err(ix) => {
                vec.insert(ix, *v);
            }
        }
    }

    vec
}