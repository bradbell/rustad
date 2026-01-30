// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 20226 Bradley M. Bell
//
// Example using the coloring algorithm for the following case:
//
// m = 4, n = 5
//
//   pattern       sub_pattern
// [ 1 0 0 0 0 ]  [ 1 0 0 0 0 ]
// [ 0 1 0 0 0 ]  [ 0 1 0 0 0 ]
// [ 1 1 1 0 0 ]  [ 1 1 1 0 0 ]
// [ 1 1 1 1 1 ]  [ 0 0 0 0 0 ]
//
// 0 = color[0] = color[1]
// 1 = color[2]
// n = color[3] = color[4] 
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
        [0,0],
        [1,1],
        [2,0], [2,1], [2,2],
        [3,0], [3,1], [3,2], [3,3], [3,4]
    ];
    let sub_pattern = vec![
        [0,0],
        [1,1],
        [2,0], [2,1], [2,2],
    ];
    let color = coloring(m, n, &pattern, &sub_pattern);
    //
    println!("color = {:?}", color);
}
