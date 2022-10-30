---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/dp.rs
    title: src/dp.rs
  - icon: ':heavy_check_mark:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dp.rs
    title: src/dp.rs
  - icon: ':heavy_check_mark:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: examples/knapsack_01.rs
    title: examples/knapsack_01.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[allow(non_snake_case)]\npub fn knapsack_01<T>(w: Vec<usize>, v: Vec<T>,\
    \ W: usize) -> T\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord\
    \ + std::default::Default,\n{\n    let mut dp = vec![T::default(); W + 1];\n \
    \   for (w, v) in w.into_iter().zip(v) {\n        for i in (w..=W).rev() {\n \
    \           dp[i] = dp[i].max(dp[i - w] + v);\n        }\n    }\n    dp[W]\n}\n"
  dependsOn:
  - src/dp.rs
  - src/lib.rs
  isVerificationFile: false
  path: src/dp/knapsack_01.rs
  requiredBy:
  - src/lib.rs
  - src/dp.rs
  timestamp: '2022-10-31 02:06:28+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - examples/knapsack_01.rs
documentation_of: src/dp/knapsack_01.rs
layout: document
title: "0-1 \u30CA\u30C3\u30D7\u30B6\u30C3\u30AF\u554F\u984C"
---

0-1 ナップザック問題を解きます．
