
pub mod algs;

fn main() {
    let s1 = "ABCBDAB";
    let s2 = "BDCABA";

    println!("Solution: {}", algs::dyn_prog::shortest_supersequence::solve(s1, s2))
}
