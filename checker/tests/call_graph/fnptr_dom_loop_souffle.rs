// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// Linear call graph with function pointer calls, single type, dominance, and a loop.
// Using the Soufflé datalog backend.

fn fn1(x: u32, fn2: &fn(u32) -> u32, fn3: &fn(u32) -> u32) -> u32 {
    let y = fn2(x);
    fn3(y)
}
fn fn2(x: u32) -> u32 {
    x + 2
}
fn fn3(x: u32) -> u32 {
    fn4(x)
}
fn fn4(x: u32) -> u32 {
    if x > 1 {
        fn3(x - 1)
    } else {
        x
    }
}
pub fn main() {
    let x = 1;
    fn1(x, &(fn2 as fn(u32) -> u32), &(fn3 as fn(u32) -> u32));
}

/* CONFIG
{
    "reductions": [],
    "included_crates": [],
    "datalog_config": {
        "datalog_backend": "Souffle"
    }
}
*/

/* EXPECTED:DOT
digraph {
    0 [ label = "\"fnptr_dom_loop_souffle::main\"" ]
    1 [ label = "\"fnptr_dom_loop_souffle::fn1\"" ]
    2 [ label = "\"fnptr_dom_loop_souffle::fn2\"" ]
    3 [ label = "\"fnptr_dom_loop_souffle::fn3\"" ]
    4 [ label = "\"fnptr_dom_loop_souffle::fn4\"" ]
    0 -> 1 [ ]
    0 -> 1 [ ]
    1 -> 2 [ ]
    1 -> 3 [ ]
    3 -> 4 [ ]
    4 -> 3 [ ]
}
*/

/* EXPECTED:SOUFFLE
2,30,0,1
1,0,1
2,1,2
3,1,3
4,3,4
5,4,30,0
1,1
2,0
3,0
4,0
5,0
*/

/* EXPECTED:TYPEMAP
{
  "0": "u32",
  "1": "&fn(u32) -> u32"
}
*/
