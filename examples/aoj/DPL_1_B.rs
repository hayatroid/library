// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B

use library::dp::knapsack_01::knapsack_01;
use library::*;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        W: usize,
        vw: [(u32, usize); n],
    }
    let w = vw.iter().map(|&p| p.1).collect::<Vec<_>>();
    let v = vw.iter().map(|&p| p.0).collect::<Vec<_>>();
    println!("{}", knapsack_01(w, v, W));
}
