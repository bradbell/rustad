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
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
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
/// //
/// let zero  = AzFloat( 0f32 );
/// let nan   = AzFloat( f32::NAN );
/// let prod  = zero * nan;
/// assert_eq!( prod, zero );
/// assert_eq!( nan == nan, true );
/// //
/// let three = AzFloat( 3f64 );
/// let four  = AzFloat( 4f64 );
/// let prod  = &three * &four;
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
// From<f32>
impl<B> From<f32> for AzFloat<B>
where
    B : From<f32> ,
{
    fn from(f : f32) -> Self {
        Self( f.into()  )
    }
}
//
// From<f64>
impl From<f64> for AzFloat<f64>
{
    fn from(f : f64) -> Self {
        Self( f.into()  )
    }
}
// ---------------------------------------------------------------------------
// AzFloat Op AzFloat
//
// Mul
impl<B> Mul for AzFloat<B>
where
    B : From<f32> + PartialEq + Mul<Output=B>,
{
    type Output = AzFloat<B>;
    fn mul(self, rhs : Self) -> Self {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                Self( zero_b )
        } else {
            Self( (self.0).mul(rhs.0) )
        }
    }
}
macro_rules! impl_binary_operator{ ($Name:ident) => { paste::paste! {
    impl<B> $Name for AzFloat<B>
    where
        B : From<f32> + PartialEq + $Name<Output=B>,
    {
        type Output = AzFloat<B>;
        fn [< $Name:lower >] (self, rhs : Self) -> Self {
            Self( (self.0). [< $Name:lower >] (rhs.0) )
        }
    }
} } }
impl_binary_operator!(Add);
impl_binary_operator!(Sub);
impl_binary_operator!(Div);
// ---------------------------------------------------------------------------
// &AzFloat Op &AzFloat
//
// Mul
impl<B> Mul<& AzFloat<B> > for &AzFloat<B>
where
    for<'a> &'a B : Mul<&'a B, Output=B>,
    B : From<f32> + PartialEq,
{
    type Output = AzFloat<B>;
    //
    fn mul(self, rhs : & AzFloat<B>) -> AzFloat<B> {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                AzFloat( zero_b )
        } else {
            AzFloat( (self.0).mul(&rhs.0) )
        }
    }
}
macro_rules! impl_binary_reference{ ($Name:ident) => { paste::paste! {
    impl<B> $Name<& AzFloat<B> > for &AzFloat<B>
    where
        for<'a> &'a B : $Name<&'a B, Output=B>,
        B : From<f32> + PartialEq ,
    {
        type Output = AzFloat<B>;
        fn [< $Name:lower >] (self, rhs : & AzFloat<B> ) -> AzFloat<B> {
            AzFloat( (self.0). [< $Name:lower >] (&rhs.0) )
        }
    }
} } }
impl_binary_reference!(Add);
impl_binary_reference!(Sub);
impl_binary_reference!(Div);
// ---------------------------------------------------------------------------
// AzFloat *= &AzFloat
impl<B> MulAssign<& AzFloat<B> > for AzFloat<B>
where
    B : From<f32> + PartialEq + for<'a> MulAssign<&'a B>,
{
    fn mul_assign(&mut self, rhs : & AzFloat<B>) {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                self.0 = zero_b;
        } else {
            self.0.mul_assign( &rhs.0 );
        }
    }
}
macro_rules! impl_binary_assign{ ($Name:ident) => { paste::paste! {
    impl<B> [< $Name Assign >] <& AzFloat<B> > for AzFloat<B>
    where
        B : From<f32> + PartialEq +  for<'a> [< $Name Assign >] <&'a B>,
    {
        fn [< $Name:lower _assign >] (& mut self, rhs : & AzFloat<B> ) {
            self.0. [< $Name:lower _assign >] ( &rhs.0 );
        }
    }
} } }
impl_binary_assign!(Add);
impl_binary_assign!(Sub);
impl_binary_assign!(Div);
// ---------------------------------------------------------------------------
// PartialEq, Eq
impl<B> PartialEq for AzFloat<B>
where
    B : PartialEq ,
{
    //
    fn eq(&self, rhs : &Self) -> bool {
        if self.is_nan() && rhs.is_nan() {
                true
        } else {
            (self.0).eq(&rhs.0)
        }
    }
}
impl<B: PartialEq> Eq for AzFloat<B> { }
