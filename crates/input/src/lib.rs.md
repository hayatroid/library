---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
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
    links:
    - https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.8/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8\n\n#[macro_export]\n\
    macro_rules! input {\n    (source = $s:expr, $($r:tt)*) => {\n        let mut\
    \ iter = $s.split_whitespace();\n        let mut next = || { iter.next().unwrap()\
    \ };\n        input_inner!{next, $($r)*}\n    };\n    ($($r:tt)*) => {\n     \
    \   let stdin = std::io::stdin();\n        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));\n\
    \        let mut next = move || -> String{\n            bytes\n              \
    \  .by_ref()\n                .map(|r|r.unwrap() as char)\n                .skip_while(|c|c.is_whitespace())\n\
    \                .take_while(|c|!c.is_whitespace())\n                .collect()\n\
    \        };\n        input_inner!{next, $($r)*}\n    };\n}\n\n#[macro_export]\n\
    macro_rules! input_inner {\n    ($next:expr) => {};\n    ($next:expr, ) => {};\n\
    \n    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {\n        let $var = read_value!($next,\
    \ $t);\n        input_inner!{$next $($r)*}\n    };\n}\n\n#[macro_export]\nmacro_rules!\
    \ read_value {\n    ($next:expr, ( $($t:tt),* )) => {\n        ( $(read_value!($next,\
    \ $t)),* )\n    };\n\n    ($next:expr, [ $t:tt ; $len:expr ]) => {\n        (0..$len).map(|_|\
    \ read_value!($next, $t)).collect::<Vec<_>>()\n    };\n\n    ($next:expr, Chars)\
    \ => {\n        read_value!($next, String).chars().collect::<Vec<char>>()\n  \
    \  };\n\n    ($next:expr, Usize1) => {\n        read_value!($next, usize) - 1\n\
    \    };\n\n    ($next:expr, $t:ty) => {\n        $next().parse::<$t>().expect(\"\
    Parse error\")\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/input/src/lib.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2022-10-31 01:01:00+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - examples/knapsack_01.rs
documentation_of: crates/input/src/lib.rs
layout: document
redirect_from:
- /library/crates/input/src/lib.rs
- /library/crates/input/src/lib.rs.html
title: crates/input/src/lib.rs
---
