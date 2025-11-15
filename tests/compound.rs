// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Testing compoound assignment operators
//
#[cfg(test)]
use rustad::{
    AzFloat,
    ad_from_value,
};
//
#[test]
fn compound() {
    //
    // V
    type V = AzFloat<f64>;
    //
    // add
    let mut ax   = ad_from_value( V::from(3.0) );
    let y        = V::from(4.0);
    ax          += &y;
    assert_eq!( ax.to_value(),  V::from(7.0) );
    //
    // sub
    let mut ax   = ad_from_value( V::from(3.0) );
    let y        = V::from(4.0);
    ax          -= &y;
    assert_eq!( ax.to_value(),  V::from(-1.0) );
    //
    // mul
    let mut ax   = ad_from_value( V::from(3.0) );
    let y        = V::from(4.0);
    ax          *= &y;
    assert_eq!( ax.to_value(),  V::from(12.0) );
    //
    // div
    let mut ax   = ad_from_value( V::from(8.0) );
    let y        = V::from(4.0);
    ax          /= &y;
    assert_eq!( ax.to_value(),  V::from(2.0) );
}
