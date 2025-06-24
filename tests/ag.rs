// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::ag::AG;

#[test]
fn from_and_to() {
    let ax  : AG<f64, u16> = AG::from( 3.0f32 );
    let x  = ax.to_value();
    assert_eq!( x, 3.0f64 );
    //
    let ax  : AG<f32, u32> = AG::from( 3.0f64 );
    let x  = ax.to_value();
    assert_eq!( x, 3.0f32 );
}
