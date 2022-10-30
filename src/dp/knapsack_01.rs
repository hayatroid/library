#[allow(non_snake_case)]
pub fn knapsack_01<T>(w: Vec<usize>, v: Vec<T>, W: usize) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord + std::default::Default,
{
    let mut dp = vec![T::default(); W + 1];
    for (w, v) in w.into_iter().zip(v) {
        for i in (w..=W).rev() {
            dp[i] = dp[i].max(dp[i - w] + v);
        }
    }
    dp[W]
}
