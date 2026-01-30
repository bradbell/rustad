// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2022-26 Bradley M. Bell
//
// Example using the coloring algorithm for the following case:
//
// m = 4, n = 5
//
//   pattern       sub_pattern
// [ 1 0 0 1 1 ]  [ 1 0 0 1 0 ]
// [ 0 1 0 1 1 ]  [ 0 1 0 1 0 ]
// [ 0 0 1 1 1 ]  [ 0 0 1 1 0 ]
// [ 0 0 0 0 1 ]  [ 0 0 0 0 0 ]
//
// 0 = color[0] = color[1] = color[2]
// 1 = color[3]
// n = color[4]
//
use rustad::{
    coloring,
};
//
fn main () {
    //
    let m = 4;
    let n = 5;
    let pattern = vec![
        [0,0], [0,3], [0,4],
        [1,1], [1,3], [1,4],
        [2,2], [2,3], [2,4],
        [3,4],
    ];
    let sub_pattern = vec![
        [0,0], [0,3], 
        [1,1], [1,3],
        [2,2], [2,3],
    ];
    let color = coloring(m, n, &pattern, &sub_pattern);
    //
    let check = vec![ 0, 0, 0, 1, n];
    assert_eq!( color, check );
}
