---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/dp.rs
    title: src/dp.rs
  - icon: ':heavy_check_mark:'
    path: src/dp/knapsack_01.rs
    title: "0-1 \u30CA\u30C3\u30D7\u30B6\u30C3\u30AF\u554F\u984C"
  - icon: ':heavy_check_mark:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B\n\
    \nuse library::knapsack_01;\nuse proconio::input;\n\n#[allow(non_snake_case)]\n\
    fn main() {\n    input! {\n        n: usize,\n        W: usize,\n        vw: [(u32,\
    \ usize); n],\n    }\n    let w = vw.iter().map(|&p| p.1).collect::<Vec<_>>();\n\
    \    let v = vw.iter().map(|&p| p.0).collect::<Vec<_>>();\n    println!(\"{}\"\
    , knapsack_01(w, v, W));\n}\n"
  dependsOn:
  - src/dp/knapsack_01.rs
  - src/dp.rs
  - src/lib.rs
  isVerificationFile: true
  path: examples/knapsack_01.rs
  requiredBy: []
  timestamp: '2022-10-31 02:06:28+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: examples/knapsack_01.rs
layout: document
redirect_from:
- /verify/examples/knapsack_01.rs
- /verify/examples/knapsack_01.rs.html
title: examples/knapsack_01.rs
---
