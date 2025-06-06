// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::OP_INFO_VEC;
use rustad::ADD_OP;

#[test]
fn test_add() {
    let mut vec : Vec<f64>   = vec![f64::NAN; 3];
    let left    : usize      = 0;
    let right   : usize      = 1;
    let res     : usize      = 2;
    let arg     : Vec<usize> = vec![left, right];
    let fun     = OP_INFO_VEC[ADD_OP].fun;
    vec[left]   = 4.0;
    vec[right]  = 5.0;
    fun(&mut vec, &arg, res);
    assert_eq!(vec[res], 9.0);
}
