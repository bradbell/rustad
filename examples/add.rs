// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::OP_INFO_VEC;
use rustad::ADD_VV_OP;
use rustad::Index;
use rustad::Float;

/// add Example
fn add_example() {
    let mut var : Vec<Float> = vec![Float::NAN; 3];
    let con     : Vec<Float> = Vec::new();
    let left    : Index      = 0;
    let right   : Index      = 1;
    let res     : Index      = 2;
    let arg     : Vec<Index> = vec![left, right];
    let fun     = OP_INFO_VEC[ADD_VV_OP].fun;
    var[left]   = 4.0;
    var[right]  = 5.0;
    fun(&mut var, &con, &arg, res);
    assert_eq!(var[res], 9.0);
}

#[test]
fn main() {
    add_example();
}
