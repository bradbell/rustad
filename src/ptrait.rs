// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! Define traits that are used by the public rustad API.
//! : [parent module](super)
//
use crate::ad::GAD;
use crate::{Float, Index};
use crate::operator::{OpInfo, ForwardZero};
// ---------------------------------------------------------------------------
/// Generic as function for converting types like `as` would.
///
/// * usize :
/// conversion to and from usize is implement for the following types:
/// u8, u16, u32, u64
///
/// # Example
/// ```
/// use rustad::ptrait::GenericAs;
/// fn use_gas<T> (f : usize) -> T where usize : GenericAs<T> {
///     GenericAs::gas(f)
/// }
/// let five    : usize = 5;
/// let convert : u32   = use_gas(five);
/// assert_eq!( 5u32, convert );
/// ```
///
pub trait GenericAs<T> {
    fn gas(self : Self) -> T;
}
//
/// Implement gas returning F as T for one (F, T) type pair.
///
/// * F : is the from type
/// * T : is the to type
///
macro_rules! generic_as{ ($F:ident, $T:ident) => {
    impl GenericAs<$T> for $F {
        fn gas(self : Self) -> $T {
            self as $T
        }
    }
} }
//
generic_as!(usize, u64);
generic_as!(usize, u32);
generic_as!(usize, u16);
generic_as!(usize, u8);
//
generic_as!(u64, usize);
generic_as!(u32, usize);
generic_as!(u16, usize);
generic_as!(u8, usize);
//
// ----------------------------------------------------------------------------
pub trait GetForwardZero<T> {
    fn get(self : &Self) -> T;
}
impl GetForwardZero< ForwardZero<Float, Index, Float> > for OpInfo {
    fn get(self : &Self) -> ForwardZero<Float, Index, Float>
    {   self.forward_0 }
}
impl GetForwardZero< ForwardZero<Float, Index, GAD<Float,Index> > > for OpInfo {
    fn get(self : &Self) -> ForwardZero<Float, Index, GAD<Float,Index> >
    {   self.ad_forward_0 }
}
// ----------------------------------------------------------------------------
use crate::record::ThisThreadTape;
//
pub trait ThisThreadTapePublic<U> : ThisThreadTape<U>
where
    U : Sized + 'static ,
{}
//
impl<F,U> ThisThreadTapePublic<U> for F
where
    F : ThisThreadTape<U> ,
    U : Sized + 'static ,
{}
