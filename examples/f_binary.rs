// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
fn main() {
    use rustad::{
        AzFloat,
        NumVec,
        FConst,
        FBinary,
    };
    //
    // num_lt
    {   type V  = AzFloat<f64>;
        let res = V::from(3.0).num_lt( V::from(4.0) );
        assert_eq!(res, FConst::one() );
    }
    //
    // num_le
    {   type V  = AzFloat<f32>;
        let res = V::from(3.0).num_le( V::from(4.0) );
        assert_eq!(res, FConst::one() );
    }
    //
    // num_eq
    {   type V  = NumVec< AzFloat<f32> >;
        let res = V::from(3.0).num_eq( V::from(4.0) );
        assert_eq!(res, FConst::zero() );
    }
    //
    // num_ne
    {   type S  = AzFloat<f64>;
        let lhs = NumVec::new( vec![ S::from(3.0), S::from(4.0) ] );
        let rhs = NumVec::new( vec![ S::from(3.0), S::from(5.0) ] );
        //
        let res   = FBinary::num_ne(&lhs, &rhs);
        let check = NumVec::new( vec![ S::from(0.0), S::from(1.0) ] );
        assert_eq!(res, check);
    }
    //
    // num_ge
    {   type S  = AzFloat<f64>;
        let lhs = NumVec::new( vec![ S::from(3.0), S::from(4.0) ] );
        let rhs = NumVec::new( vec![ S::from(3.0), S::from(5.0) ] );
        //
        let res   = FBinary::num_ge(&lhs, &rhs);
        let check = NumVec::new( vec![ S::from(1.0), S::from(0.0) ] );
        assert_eq!(res, check);
    }
    //
    // num_gt
    {   type S  = AzFloat<f32>;
        let lhs = NumVec::new( vec![ S::from(3.0), S::from(4.0) ] );
        let rhs = NumVec::new( vec![ S::from(3.0), S::from(5.0) ] );
        //
        let res   = lhs.num_gt(rhs);
        let check = NumVec::new( vec![ S::from(0.0), S::from(0.0) ] );
        assert_eq!(res, check);
    }
    //
    // powf
    {   type S  = AzFloat<f32>;
        let lhs = NumVec::new( vec![ S::from(1.0), S::from(2.0) ] );
        let rhs = NumVec::new( vec![ S::from(3.0), S::from(4.0) ] );
        //
        let res   = lhs.powf(rhs);
        let check = NumVec::new( vec![ S::from(1.0), S::from(16.0) ] );
        assert_eq!(res, check);
    }
    //
    // hypot
    {   type V  = AzFloat<f32>;
        let lhs = V::from(3.0);
        let rhs = V::from(4.0);
        let res = lhs.hypot(rhs);
        assert_eq!(res, V::from(5.0));
    }
}
