// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// Linear call graph with static calls, single type, no dominance, no loops.
// Taking a slice of the call graph from fn1.

pub fn fn1(x: u32) -> u32 {
    fn2(x)
}
fn fn2(x: u32) -> u32 {
    fn3(x)
}
fn fn3(x: u32) -> u32 {
    x
}
pub fn main() {
    let x = 1;
    fn1(x);
}

/* CONFIG
{
    "reductions": [{"Slice": "fn1"}],
    "included_crates": [],
    "datalog_config": {
        "datalog_backend": "DifferentialDatalog"
    }
}
*/

/* EXPECTED:DOT
digraph {
    0 [ label = "\"static_slice::fn1\"" ]
    1 [ label = "\"static_slice::fn2\"" ]
    2 [ label = "\"static_slice::fn3\"" ]
    0 -> 1 [ ]
    1 -> 2 [ ]
}
*/

/* EXPECTED:DDLOG
start;
insert Edge(0,0,1);
insert Edge(1,1,2);
insert EdgeType(0,0);
insert EdgeType(1,0);
commit;
*/

/* EXPECTED:TYPEMAP
{
  "0": "u32"
}
*/
