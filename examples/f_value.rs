// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
// az_float_example
fn az_float_example() {
    use rustad::{
        AzFloat,
        FConst,
        FValue,
    };
    // is_zero
    {   type V      = AzFloat<f64>;
        let zero    = V::zero();
        assert!( zero.is_zero() );
        let one    = V::one();
        assert!( ! one.is_zero() );
    }
    // is_one
    {   type V      = AzFloat<f32>;
        let zero    = V::zero();
        assert!( ! zero.is_one() );
        let one    = V::one();
        assert!( one.is_one() );
    }
    // is_nan
    {   type V      = AzFloat<f64>;
        let zero    = V::zero();
        assert!( ! FValue::is_nan( &zero ) );
        let nan     = V::nan();
        assert!( FValue::is_nan( &nan ) );
    }
    // to_src
    {   type V  = AzFloat<f32>;
        let two = V::from(2.0);
        assert_eq!( two.to_src(), "AzFloat(2 as f32)" );
    }
}
//
// main
fn main() {
    az_float_example();
}
