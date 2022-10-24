/******************************** begin Knapsack-01 ********************************/
#[allow(dead_code, non_snake_case)]
fn knapsack_01<T>(wv: Vec<(usize, T)>, W: usize) -> T
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
/******************************** end Knapsack-01 ********************************/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atcoder_01() {
        let wv = vec![(3, 30), (4, 50), (5, 60)];
        let w = 8;
        assert_eq!(knapsack_01(wv, w), 90);
    }
    #[test]
    fn atcoder_02() {
        let wv = vec![
            (1, 1000000000),
            (1, 1000000000),
            (1, 1000000000),
            (1, 1000000000),
            (1, 1000000000),
        ];
        let w = 5;
        assert_eq!(knapsack_01(wv, w), 5000000000_u64);
    }
    #[test]
    fn atcoder_03() {
        let wv = vec![(6, 5), (5, 6), (6, 4), (6, 6), (3, 5), (7, 2)];
        let w = 15;
        assert_eq!(knapsack_01(wv, w), 17)
    }
}
