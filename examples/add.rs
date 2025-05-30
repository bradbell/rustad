// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad;

/// add Example
fn add_example() {
    let mut vec : Vec<f64> = vec![f64::NAN; 3];
    let left    = 0;
    let right   = 1;
    let result  = 2;
    let fun     = rustad::OP_INFO_VEC[rustad::ADD_OP].fun;
    vec[left]  = 4.0;
    vec[right] = 5.0;
    fun(&mut vec, left, right, result);
    assert_eq!(vec[result], 9.0);
}

#[test]
fn main() {
    add_example();
}
