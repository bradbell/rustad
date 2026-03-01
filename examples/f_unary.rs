// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
fn main() {
    use rustad::{
        AzFloat,
        NumVec,
        AD,
        FConst,
        FUnary,
        check_nearly_eq,
    };
    //
    // ----------------------------------------------------------------------
    // No Arguments
    // ----------------------------------------------------------------------
    // pi
    {   type V      = AzFloat<f64>;
        let pi : V  = FConst::pi();
        let check   = V::from(3.14159265358979323846264338327950288);
        assert_eq!(pi, check);
    }
    // nan
    {   type V      = AzFloat<f32>;
        let nan : V = FConst::nan();
        // AzFloat defines nan as equal to nan
        assert_eq!( nan, nan );
        // f32 defines nan as not equal to nan
        assert_ne!( nan.to_inner(), nan.to_inner() )
    }
    // one
    {   type V          = AzFloat<f32>;
        let one : AD<V> = FConst::one();
        let two : AD<V> = V::from(2).into();
        assert_eq!( V::from(2), ( &two * &one).to_value() );
    }
    // zero
    {   type S       = AzFloat<f64>;
        type V       = NumVec<S>;
        let zero : V = FConst::zero();
        let two  : V = NumVec::from( S::from(2) );
        assert_eq!( two, &two + &zero );
    }
    // epsilon
    {   type V      = AzFloat<f64>;
        let epsilon : V = FConst::epsilon();
        let one     : V = FConst::one();
        let two     : V = V::from(2);
        assert_ne!( one, one + epsilon );
        assert_eq!( one, one + epsilon / two );
    }
    // min_positive
    {   type V               = AzFloat<f32>;
        let min_positive : V = FConst::min_positive();
        let epsilon      : V = FConst::epsilon();
        let zero         : V = FConst::zero();
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
        assert_eq!( sum, FConst::zero() );
    }
    // exp
    {   type V = AzFloat<f32>;
        let one          = FConst::one();
        let exp_3        = FUnary::exp( &V::from(3.0) );
        let exp_minus_3  = FUnary::exp( &V::from(-3.0) );
        let prod         = exp_3 * exp_minus_3;
        assert_eq!(prod, one);
    }
    // minus
    {   type V = AzFloat<f64>;
        let three        = V::from(3.0);
        let minus_3      = three.minus();
        let sum          = three + minus_3;
        assert_eq!( sum, FConst::zero() );
    }
    // cos
    {   type V = AzFloat<f64>;
        let cos_0        = FUnary::cos( &V::from(0.0) );
        assert_eq!( cos_0, FConst::one() );
    }
    // cosh
    {   type V = AzFloat<f64>;
        let two         = V::from(2.0);
        let cosh_2      = FUnary::cosh( &two );
        //
        let exp_2       = two.exp();
        let exp_minus_2 = two.minus().exp();
        let check       = ( exp_2 + exp_minus_2 ) / two;
        assert_eq!(cosh_2, check);
    }
    // ln
    {   type V = AzFloat<f64>;
        let two         = V::from(2.0);
        let ln_2       = two.ln();
        //
        let exp_ln_2   = ln_2.exp();
        assert_eq!(exp_ln_2, two);
    }
    // signum
    {   type V = AzFloat<f32>;
        let minus_1      = V::from( -1.0 );
        let minus_3      = V::from( -3.0 );
        assert_eq!( minus_3.signum(), minus_1);
    }
    // sin
    {   type V = AzFloat<f64>;
        let pi : V   = FConst::pi();
        let pi_2     = pi / V::from(2);
        let sin_pi_2 = pi_2.sin();
        assert_eq!(sin_pi_2, FConst::one());
    }
    // sinh
    {   type V = AzFloat<f64>;
        let two         = V::from(2.0);
        let sinh_2      = FUnary::sinh( &two );
        //
        let exp_2       = two.exp();
        let exp_minus_2 = two.minus().exp();
        let check       = ( exp_2 - exp_minus_2 ) / two;
        assert_eq!(sinh_2, check);
    }
    // sqrt
    {   type V = AzFloat<f64>;
        let two         = V::from(2.0);
        let four        = V::from(4.0);
        let sqrt_4      = FUnary::sqrt( &four);
        assert_eq!(sqrt_4, two);
    }
    // tan
    {   type V = AzFloat<f32>;
        let pi : V   = FConst::pi();
        let pi_4     = pi / V::from(4);
        let tan_pi_4 = FUnary::tan( &pi_4 );
        assert_eq!(tan_pi_4, FConst::one());
    }
    // tanh
    {   type V = AzFloat<f32>;
        let tanh_2      = V::from(2.0).tanh();
        let exp_2       = V::from(2.0).exp();
        let exp_minus_2 = V::from(-2.0).exp();
        let check       = (exp_2 - exp_minus_2) / (exp_2 + exp_minus_2);
        let arg_vec : Vec<[&str; 2]> = Vec::new();
        check_nearly_eq::<V>(&tanh_2, &check, &arg_vec);
    }
    // ----------------------------------------------------------------------
    // powi
    {   type S = AzFloat<f64>;
        type V = NumVec<S>;
        let pair        = V::new( vec![ S::from(2), S::from(3) ] );
        let pair_pow_2  = pair.powi(2);
        assert_eq!( pair_pow_2.get(0), S::from(4) );
        assert_eq!( pair_pow_2.get(1), S::from(9) );
    }
}
