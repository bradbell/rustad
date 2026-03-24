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
// num_vec_example
fn num_vec_example() {
    use rustad::{
        NumVec,
        AzFloat,
        FConst,
        FValue,
    };
    // is_zero
    {   type S      = AzFloat<f64>;
        let zero_one   = NumVec::new( vec![ S::zero(), S::one() ] );
        assert!( ! zero_one.is_zero() );
        let zero    = NumVec::new( vec![ S::zero(), S::zero() ] );
        assert!(  zero.is_zero() );
    }
    // is_one
    {   type S      = AzFloat<f64>;
        let zero_one   = NumVec::new( vec![ S::zero(), S::one() ] );
        assert!( ! zero_one.is_zero() );
        let one     = NumVec::new( vec![ S::one(), S::one() ] );
        assert!(  one.is_one() );
    }
    // is_nan
    {   type S     = AzFloat<f64>;
        type V     = NumVec<S>;
        let zero   = V::zero();
        assert!( ! zero.is_nan() );
        let nan    = V::nan();
        assert!( FValue::is_nan( &nan ) );
    }
    // to_src
    {   type S       = AzFloat<f32>;
        let zero_one = NumVec::new( vec![ S::zero(), S::one() ] );
        let check =
            "NumVec::new( vec![ AzFloat(0 as f32), AzFloat(1 as f32), ] )";
        assert_eq!( zero_one.to_src(), check);
    }
}
//
// main
fn main() {
    az_float_example();
    num_vec_example();
}
