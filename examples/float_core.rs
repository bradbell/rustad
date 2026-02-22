// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
fn main() {
    use rustad::{
        AzFloat,
        NumVec,
        AD,
        FloatCore,
        check_nearly_eq,
    };
    // arg_vec
    let arg_vec = vec![ ["assert", "true"] ];
    //
    // ----------------------------------------------------------------------
    // No Arguments
    // ----------------------------------------------------------------------
    // nan
    {   type V      = AzFloat<f32>;
        let nan : V = FloatCore::nan();
        // AzFloat defines nan as equal to nan
        assert_eq!( nan, nan );
        // f32 defines nan as not equal to nan
        assert_ne!( nan.to_inner(), nan.to_inner() )
    }
    // zero
    {   type S       = AzFloat<f64>;
        type V       = NumVec<S>;
        let zero : V = FloatCore::zero();
        let two  : V = NumVec::from( S::from(2) );
        assert_eq!( two, &two + &zero );
    }
    // one
    {   type V          = AzFloat<f32>;
        let one : AD<V> = FloatCore::one();
        let two : AD<V> = V::from(2).into();
        assert_eq!( V::from(2), ( &two * &one).to_value() );
    }
    // epsilon
    {   type V      = AzFloat<f64>;
        let epsilon : V = FloatCore::epsilon();
        let one     : V = FloatCore::one();
        let two     : V = V::from(2);
        assert_ne!( one, one + epsilon );
        assert_eq!( one, one + epsilon / two );
    }
    // min_positive
    {   type V               = AzFloat<f32>;
        let min_positive : V = FloatCore::min_positive();
        let epsilon      : V = FloatCore::epsilon();
        let zero         : V = FloatCore::zero();
        let two          : V = V::from(2);
        assert!( zero < min_positive * epsilon );
        assert_eq!( zero , min_positive * epsilon / two );
    }
    // ----------------------------------------------------------------------
    // No Arguments
    // ----------------------------------------------------------------------
    // abs
    {   type V = AzFloat<f64>;
        let minus_3      = V::from( - 3.0 );
        let abs_minus_3  = minus_3.abs();
        let sum          = minus_3 + abs_minus_3;
        assert_eq!( sum, FloatCore::zero() );
    }
    // exp
    {   type V = AzFloat<f32>;
        let one          = FloatCore::one();
        let exp_3        = FloatCore::exp( &V::from(3.0) );
        let exp_minus_3  = FloatCore::exp( &V::from(-3.0) );
        let prod         = exp_3 * exp_minus_3;
        check_nearly_eq::<V>(&prod, &one, &arg_vec);
    }
    // minus
    {   type V = AzFloat<f64>;
        let three        = V::from(3.0);
        let minus_3      = three.minus();
        let sum          = three + minus_3;
        assert_eq!( sum, FloatCore::zero() );
    }
    // cos
    {   type V = AzFloat<f64>;
        let cos_0        = FloatCore::cos( &V::from(0.0) );
        assert_eq!( cos_0, FloatCore::one() );
    }
    // signum
    {   type V = AzFloat<f32>;
        let minus_1      = V::from( -1.0 );
        let minus_3      = V::from( -3.0 );
        assert_eq!( minus_3.signum(), minus_1);
    }
    // sin
    {   type V = AzFloat<f64>;
        let pi       = V::from(3.14159265358979323846264338327950288419716939);
        let pi_2     = pi / V::from(2);
        let sin_pi_2 = pi_2.sin();
        check_nearly_eq::<V>(&sin_pi_2, &FloatCore::one(), &arg_vec);
    }
}
