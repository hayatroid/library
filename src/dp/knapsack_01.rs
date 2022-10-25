#[allow(dead_code, non_snake_case)]
pub fn knapsack_01<T>(wv: Vec<(usize, T)>, W: usize) -> T
where
    T: Copy + Clone + std::ops::Add<Output = T> + std::cmp::Ord + std::default::Default,
{
    let mut dp = vec![T::default(); W + 1];
    for (w, v) in wv {
        for i in (w..=W).rev() {
            dp[i] = dp[i].max(dp[i - w] + v);
        }
    }
    dp[W]
}
