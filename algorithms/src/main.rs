use crate::algs::dyn_prog::longest_subsequence::solve;

pub mod algs;

fn main() {
    let s1 = "ABCBDAB";
    let s2 = "BDCABA";

    println!("Solution: {}", solve(s1, s2))
}
