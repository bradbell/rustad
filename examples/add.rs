// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::OP_INFO_VEC;
use rustad::ADD_OP;
use rustad::Index;

/// add Example
fn add_example() {
    let mut vec : Vec<f64>   = vec![f64::NAN; 3];
    let left    : Index      = 0;
    let right   : Index      = 1;
    let res     : Index      = 2;
    let arg     : Vec<Index> = vec![left, right];
    let fun     = OP_INFO_VEC[ADD_OP].fun;
    vec[left]   = 4.0;
    vec[right]  = 5.0;
    fun(&mut vec, &arg, res);
    assert_eq!(vec[res], 9.0);
}

#[test]
fn main() {
    add_example();
}
