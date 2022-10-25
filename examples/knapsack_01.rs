// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B

use library::dp::knapsack_01::knapsack_01;
use library::{input, input_inner, read_value};

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        W: usize,
        vw: [(u32, usize); n],
    }
    let w = vw.iter().map(|&p| p.1).collect::<Vec<usize>>();
    let v = vw.iter().map(|&p| p.0).collect::<Vec<u32>>();
    println!("{}", knapsack_01(w, v, W));
}
