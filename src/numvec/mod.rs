// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module defines the numeric vector class `NumVec` < *S* >.
//!
//! Link to [parent module](super)
//!
//! * S :
//! is the scalar type; i.e., the type of elements of the vector.
//!
//! * Scalars :
//! If a NunVec has one element, it acts like a scalar; i.e.,
//! a vector with any required length and all the element
//! equal to the scalar value.
//!
//! * Copy, Clone :
//!  The NumVec types implement Clone, but not the Copy trait.
// ---------------------------------------------------------------------------
// sub-modules
//
// ad
pub mod ad;
//
// tape
pub mod tape;
//
// adfn
pub mod adfn;
//
// op
pub(crate) mod op;
// ---------------------------------------------------------------------------
// Sealed Traits
//
// ThisThreadTapePublic
/// This is the public interface to a sealed trait
pub trait ThisThreadTapePublic : tape::sealed::ThisThreadTape
{ }
impl<V> ThisThreadTapePublic for V
where
    V : tape::sealed::ThisThreadTape ,
{ }
// ---------------------------------------------------------------------------
// re-export
//
pub use ad::{
    AD,
    ad_from_value,
};
pub use adfn::{
    ADFn,
};
pub use tape::{
    start_recording,
    stop_recording,
};
//
// TODO: remove these when GlobalOpInfoVec gets used
pub use op::info::{
    GlobalOpInfoVec,
    OpInfo
};
// ---------------------------------------------------------------------------
/// Set up rustad to do calculations with value type V; see
/// [doc_generic_v](crate::numvec::ad::doc_generic_v) .
///
/// /// This macro must be executed once for any type *V*  where
/// `AD<V>` is used.
/// The rustad package automatically executes this macro
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
macro_rules! setup_this_value_type{ ($V:ty) => {
        crate::numvec::tape::impl_this_thread_tape!($V);
        crate::numvec::op::info::impl_global_op_info_vec!($V);
        crate::numvec::ad::impl_value_op_ad!($V);
} }
setup_this_value_type!(f32);
setup_this_value_type!(f64);
setup_this_value_type!( NumVec<f32> );
setup_this_value_type!( NumVec<f64> );
// ---------------------------------------------------------------------------
//
// NumVec
/// The numeric vector type
#[derive(Debug,Clone)]
pub struct NumVec<S> {
    /// The elements of this numeric vector
    pub vec : Vec<S> ,
}
//
// new
impl<S> NumVec<S>
{   //
    /// Create a new numeric vector using the specified data
    pub fn new( v : Vec<S> ) -> Self {
        Self { vec: v }
    }
}
//
// len
impl<S> NumVec<S>
{   //
    /// Length of this numeric vector
    pub fn len(self : &NumVec<S> ) -> usize {
        self.vec.len()
    }
}
// ---------------------------------------------------------------------------
/// Binary `NumVec` < *S* > operators.
///
/// S : is the type of the elements of the numeric vector.
///
/// Op : is the source code token for this binary operator;
/// i.e., `+` , `-` , `*` , or `/` .
///
/// Prototype:
/// <br/>
/// & `NumVec` < *S* > *Op* & `NumVec` < *S* >
///
/// # Example
///```
/// use rustad::numvec::NumVec;
///
/// let a  = NumVec::new( vec![ 1f64, 2f64 ] );
/// let b  = NumVec::new( vec![ 3f64 ] );
/// let c  = NumVec::new( vec![ 4f64 ] );
///
/// let d = &a * &b;
/// assert_eq!( d.len(), 2);
/// assert_eq!( d.vec[0], 3f64 );
/// assert_eq!( d.vec[1], 6f64 );
///
/// let d = &b - &c;
/// assert_eq!( d.len(), 1);
/// assert_eq!( d.vec[0], -1.0f64 );
/// ```
pub fn doc_numvec_binary_op() { }
//
/// Add one binary operator to the `NumVec` < *S* > class;
/// see [doc_numvec_binary_op]
macro_rules! numvec_binary_op { ($Name:ident, $Op:tt) => { paste::paste! {

    #[doc = concat!(
        "& `NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_numvec_binary_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for &'a NumVec<S>
    where
        S : Copy + std::ops::$Name<Output=S> ,
    {   type Output = NumVec<S>;
        //
        fn [< $Name:lower >](self : &'a NumVec<S>, rhs : &'a NumVec<S> )
        -> NumVec<S>
        {   let mut v : Vec<S>;
            if self.len() == 1 {
                v = rhs.vec.clone();
                if rhs.len() == 1 {
                    v[0] = self.vec[0] $Op rhs.vec[0];
                } else {
                    for j in 0 .. rhs.len() {
                        v[j] = self.vec[0] $Op rhs.vec[j];
                    }
                }
            } else {
                v = self.vec.clone();
                if rhs.len() == 1 {
                    for j in 0 .. self.len() {
                        v[j] = self.vec[j] $Op rhs.vec[0];
                    }
                } else {
                    assert_eq!( self.len(), rhs.len() );
                    for j in 0 .. self.len() {
                        v[j] = self.vec[j] $Op rhs.vec[j];
                    }
                }
            }
            NumVec::new(v)
        }
    }
} } }
//
numvec_binary_op!(Add, +);
numvec_binary_op!(Sub, -);
numvec_binary_op!(Mul, *);
numvec_binary_op!(Div, /);
// ----------------------------------------------------------------------------`
/// Compound Assignment `NumVec` < *S* > operators.
///
/// S : is the scalar type; i.e., type of the elements of the numeric vector.
///
/// Op : is the source code token for this binary operator;
/// i.e., `+=` , `-=` , `*=` , or `/=` .
///
/// Prototype:
/// <br/>
/// & `NumVec` < *S* > *Op* & `NumVec` < *S* >
///
/// # Example
///```
/// use rustad::numvec::NumVec;
///
/// let mut a = NumVec::new( vec![ 12f64, 6f64 ] );
/// let mut b = NumVec::new( vec![ 3f64 ] );
/// let c     = NumVec::new( vec![ 4f64 ] );
///
/// a /= &b;
/// assert_eq!( a.len(), 2);
/// assert_eq!( a.vec[0], 4f64 );
/// assert_eq!( a.vec[1], 2f64 );
///
/// b += &c;
/// assert_eq!( b.len(), 1);
/// assert_eq!( b.vec[0], 7.0f64 );
/// ```
pub fn doc_numvec_compound_op() { }
//
/// Add one compound assignment operator to the `NumVec` < *S* > class;
/// see [doc_numvec_compound_op]
macro_rules! numvec_compound_op { ($Name:ident, $Op:tt) => { paste::paste! {

    #[doc = concat!(
        "`NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_numvec_compound_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for NumVec<S>
    where
        S : Copy + std::ops::$Name ,
    {   //
        fn [< $Name:snake >] (&mut self, rhs : &'a NumVec<S> )
        {   //
            if self.len() == 1 {
                if rhs.len() == 1 {
                    self.vec[0] $Op rhs.vec[0];
                } else {
                    self.vec = vec![ self.vec[0] ; rhs.len() ];
                    for j in 0 .. rhs.len() {
                        self.vec[j] $Op rhs.vec[j];
                    }
                }
            } else {
                if rhs.len() == 1 {
                    for j in 0 .. self.len() {
                        self.vec[j] $Op rhs.vec[0];
                    }
                } else {
                    assert_eq!( self.len(), rhs.len() );
                    for j in 0 .. self.len() {
                        self.vec[j] $Op rhs.vec[j];
                    }
                }
            }
        }
    }
} } }
//
numvec_compound_op!(AddAssign, +=);
numvec_compound_op!(SubAssign, -=);
numvec_compound_op!(MulAssign, *=);
numvec_compound_op!(DivAssign, /=);
// ----------------------------------------------------------------------------`
/// Displays a `NumVec` < *S* > object.
///
/// The text "[ " and " ]" surround the elements of the vector.
/// The elements are separated by ", "
/// and there is a "," after the last element.
///
/// # Example using NumVec
/// ```
/// use rustad::numvec::AD;
/// use rustad::numvec::ad_from_value;
/// use rustad::numvec::NumVec;
/// let x     : Vec<f64>  = vec![5.0, 6.0];
/// let x_nv              = NumVec::new(x);
/// let s                 = format!( "{x_nv}" );
/// assert_eq!( s, "[ 5, 6, ]" );
/// ```
impl<S : std::fmt::Display> std::fmt::Display for NumVec<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")?;
        for j in 0 .. self.len() {
            write!(f, "{}, ", self.vec[j])?;
        }
        write!(f, "]")
    }
}
