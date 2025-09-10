// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Testing compoound assignment operators
//
use rustad::AD;
//
#[test]
fn test_compoundd_ad() {
    //
    // add
    let mut ax   = AD::from( 3.0f64 );
    let y        = 4.0f64;
    ax          += &y;
    assert_eq!( ax.to_value(),  7.0 );
    //
    // sub
    let mut ax   = AD::from( 3.0f64 );
    let y        = 4.0f64;
    ax          -= &y;
    assert_eq!( ax.to_value(),  -1.0 );
    //
    // mul
    let mut ax   = AD::from( 3.0f64 );
    let y        = 4.0f64;
    ax          *= &y;
    assert_eq!( ax.to_value(),  12.0 );
    //
    // div
    let mut ax   = AD::from( 8.0f64 );
    let y        = 4.0f64;
    ax          /= &y;
    assert_eq!( ax.to_value(),  2.0 );
}
