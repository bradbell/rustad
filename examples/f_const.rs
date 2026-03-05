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
    };
    // pi
    {   type V      = AzFloat<f64>;
        let pi      = V::pi();
        let check   = V::from(3.14159265358979323846264338327950288);
        assert_eq!(pi, check);
    }
    // nan
    {   type V      = AzFloat<f32>;
        let nan     = V::nan();
        // AzFloat defines nan as equal to nan
        assert_eq!( nan, nan );
        // f32 defines nan as not equal to nan
        assert_ne!( nan.to_inner(), nan.to_inner() )
    }
    // one
    {   type V          = AzFloat<f32>;
        let one         = AD::<V>::one();
        let two : AD<V> = V::from(2).into();
        assert_eq!( V::from(2), ( &two * &one).to_value() );
    }
    // zero
    {   type S       = AzFloat<f64>;
        type V       = NumVec<S>;
        let zero     = V::zero();
        let two  : V = NumVec::from( S::from(2) );
        assert_eq!( two, &two + &zero );
    }
    // epsilon
    {   type V      = AzFloat<f64>;
        let epsilon     = V::epsilon();
        let one         = V::one();
        let two     : V = V::from(2);
        assert_ne!( one, one + epsilon );
        assert_eq!( one, one + epsilon / two );
    }
    // min_positive
    {   type V               = AzFloat<f32>;
        let min_positive     = V::min_positive();
        let epsilon          = V::epsilon();
        let zero             = V::zero();
        let two          : V = V::from(2);
        assert!( zero < min_positive * epsilon );
        assert_eq!( zero , min_positive * epsilon / two );
    }
}
