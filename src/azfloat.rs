// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the rustad AzFloat class.
//!
//! Link to [parent module](super)
//!
//!
// ---------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul
};
//
// ---------------------------------------------------------------------------
/// The Absolute Zero Floating point class
///
/// B : the base floating point class is either f32 or f64
///
/// This is acts like the base class with the following different properties:
///
/// * : zero is an absolute zero; i.e. multiplication by zero
/// always results in zero (even if the other operand is nan).
///
/// * : nan is equal to nan.
///
/// # Example
/// ```
/// use rustad::AzFloat;
///
/// let zero  = AzFloat( 0f32 );
/// let nan   = AzFloat( f32::NAN );
/// let prod  = zero * nan;
/// assert_eq!( prod, zero );
/// assert_eq!( nan == nan, true );
///
/// let three = AzFloat( 3f64 );
/// let four  = AzFloat( 4f64 );
/// let prod  = three * four;
/// assert_eq!( prod.to_inner(), 12f64 );
///
#[derive(Debug, Clone, Copy)]
pub struct AzFloat<B>(pub B);
//
impl<B> AzFloat<B>
where
    B : PartialEq ,
{
    //
    // is_nan
    pub fn is_nan(&self) -> bool {
        self.0 != self.0
    }
    //
    // to_inner
    pub fn to_inner(self) -> B {
        self.0
    }
}
//
// Mul
impl<B> Mul for AzFloat<B>
where
    B : From<f32> + PartialEq + Mul<Output=B>,
{
    type Output = AzFloat<B>;
    //
    fn mul(self, other : Self) -> Self {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (other.0) == zero_b {
                Self( zero_b )
        } else {
            Self( (self.0).mul(other.0) )
        }
    }
}
//
// PartialEq, Eq
impl<B> PartialEq for AzFloat<B>
where
    B : PartialEq ,
{
    //
    fn eq(&self, other : &Self) -> bool {
        if self.is_nan() && other.is_nan() {
                true
        } else {
            (self.0).eq(&other.0)
        }
    }
}
impl<B: PartialEq> Eq for AzFloat<B> { }
