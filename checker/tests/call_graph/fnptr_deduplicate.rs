// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// Linear call graph with function pointer calls, single type, no dominance, no loops.
// Functions have two types, after deduplication only one edge should exist.

fn fn1(x: u32, fn2: &fn(u32) -> u32) -> u32 {
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
    fn1(x, &(fn2 as fn(u32) -> u32));
}

/* CONFIG
{
    "reductions": ["Deduplicate"],
    "included_crates": [],
    "datalog_config": {
        "datalog_backend": "DifferentialDatalog"
    }
}
*/

/* EXPECTED:DOT
digraph {
    0 [ label = "\"fnptr_deduplicate::main\"" ]
    1 [ label = "\"fnptr_deduplicate::fn1\"" ]
    2 [ label = "\"fnptr_deduplicate::fn2\"" ]
    3 [ label = "\"fnptr_deduplicate::fn3\"" ]
    0 -> 1 [ ]
    1 -> 2 [ ]
    2 -> 3 [ ]
}
*/

/* EXPECTED:DDLOG
start;
insert Edge(0,0,1);
insert Edge(1,1,2);
insert Edge(2,2,3);
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
