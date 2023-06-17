mod prob_7;
mod prob_4;
mod prob_3;
mod prob_2;
mod prob_1;
mod util;

fn main() {

    prob_7::solve(10001, true);
    // prob_4::solve(100, 999, Some(true));
    assert!(prob_7::is_prime(104743));
}
