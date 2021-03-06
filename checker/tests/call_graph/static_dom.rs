// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// Linear call graph with static calls, single type, dominance, no loops.

fn fn1(x: u32) -> u32 {
    x + 1
}
fn fn2(x: u32) -> u32 {
    x + 2
}
fn fn3(x: u32) -> u32 {
    x + 3
}
pub fn main() {
    let x = 1;
    let y = fn1(x);
    let z = fn2(y);
    fn3(z);
}

/* CONFIG
{
    "reductions": [],
    "included_crates": [],
    "datalog_config": {
        "datalog_backend": "DifferentialDatalog"
    }
}
*/

/* EXPECTED:DOT
digraph {
    0 [ label = "\"static_dom::main\"" ]
    1 [ label = "\"static_dom::fn1\"" ]
    2 [ label = "\"static_dom::fn2\"" ]
    3 [ label = "\"static_dom::fn3\"" ]
    0 -> 1 [ ]
    0 -> 2 [ ]
    0 -> 3 [ ]
}
*/

/* EXPECTED:DDLOG
start;
insert Dom(1,2);
insert Dom(1,3);
insert Dom(2,3);
insert Edge(0,0,1);
insert Edge(1,0,2);
insert Edge(2,0,3);
insert EdgeType(0,0);
insert EdgeType(1,0);
insert EdgeType(2,0);
commit;
*/

/* EXPECTED:TYPEMAP
{
  "0": "u32"
}
*/
