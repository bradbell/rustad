// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the numeric vector class `NumVec` < *S* >.
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
// use
use ordered_float::OrderedFloat;
use crate::AzFloat;
//
// NumVec
/// The numeric vector type
#[derive(Debug,Clone)]
pub struct NumVec<S> {
    /// The elements of this numeric vector
    vec : Vec<S> ,
    /// Value if this vector has only one element
    s   : S ,
}
//
// new
impl<S> NumVec<S>
where
    S : From<f32> + Copy,
{   //
    /// Create a new numeric vector using the specified data
    pub fn new( v : Vec<S> ) -> Self {
        assert_ne!( v.len(), 0);
        if v.len() == 1 {
            Self { vec: Vec::new() , s : v[0]}
        } else {
            Self { vec: v , s : f32::NAN.into() }
        }
    }
}
//
// len
impl<S> NumVec<S>
{   //
    /// Length of this numeric vector
    pub fn len(self : &NumVec<S> ) -> usize {
        if self.vec.len() == 0 {
            1
        } else {
            self.vec.len()
        }
    }
}
//
// get
impl<S> NumVec<S>
where
    S : Copy ,
{   //
    /// get an element of a numeric vector
    pub fn get(&self, index : usize) -> S
    {   if self.len() == 1 {
            debug_assert!( index == 0);
            self.s
        } else {
            self.vec[index]
        }
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
/// use rustad::NumVec;
///
/// let a  = NumVec::new( vec![ 1f64, 2f64 ] );
/// let b  = NumVec::from( 3f64 );
/// let c  = NumVec::from( 4f64 );
///
/// let d = &a * &b;
/// assert_eq!( d.len(), 2);
/// assert_eq!( d.get(0), 3f64 );
/// assert_eq!( d.get(1), 6f64 );
///
/// let d = &b - &c;
/// assert_eq!( d.len(), 1);
/// assert_eq!( d.get(0), -1.0f64 );
/// ```
pub fn doc_num_vec_binary_op() { }
//
/// Add one binary operator to the `NumVec` < *S* > class;
/// see [doc_num_vec_binary_op]
macro_rules! num_vec_binary_op { ($Name:ident, $Op:tt) => { paste::paste! {

    #[doc = concat!(
        "& `NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_num_vec_binary_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for &'a NumVec<S>
    where
        S : From<f32> + Copy + std::ops::$Name<Output=S> ,
    {   type Output = NumVec<S>;
        //
        fn [< $Name:lower >](self : &'a NumVec<S>, rhs : &'a NumVec<S> )
        -> NumVec<S>
        {   let mut v : Vec<S>;
            let e     : S;
            if self.len() == 1 {
                if rhs.len() == 1 {
                    e = self.s $Op rhs.s;
                    v = Vec::new();
                } else {
                    e = f32::NAN.into();
                    v = vec![e; rhs.len() ];
                    for j in 0 .. rhs.len() {
                        v[j] = self.s $Op rhs.vec[j];
                    }
                }
            } else {
                e = f32::NAN.into();
                v = vec![e; self.len() ];
                if rhs.len() == 1 {
                    for j in 0 .. self.len() {
                        v[j] = self.vec[j] $Op rhs.s;
                    }
                } else {
                    assert_eq!( self.len(), rhs.len() );
                    for j in 0 .. self.len() {
                        v[j] = self.vec[j] $Op rhs.vec[j];
                    }
                }
            }
            NumVec{ vec : v, s : e }
        }
    }
} } }
//
num_vec_binary_op!(Add, +);
num_vec_binary_op!(Sub, -);
num_vec_binary_op!(Mul, *);
num_vec_binary_op!(Div, /);
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
/// use rustad::NumVec;
///
/// let mut a = NumVec::new( vec![ 12f64, 6f64 ] );
/// let mut b = NumVec::from( 3f64 );
/// let c     = NumVec::from( 4f64 );
///
/// a /= &b;
/// assert_eq!( a.len(), 2);
/// assert_eq!( a.get(0), 4f64 );
/// assert_eq!( a.get(1), 2f64 );
///
/// b += &c;
/// assert_eq!( b.len(), 1);
/// assert_eq!( b.get(0), 7.0f64 );
/// ```
pub fn doc_num_vec_compound_op() { }
//
/// Add one compound assignment operator to the `NumVec` < *S* > class;
/// see [doc_num_vec_compound_op]
macro_rules! num_vec_compound_op { ($Name:ident, $Op:tt) => { paste::paste! {

    #[doc = concat!(
        "`NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_num_vec_compound_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for NumVec<S>
    where
        S : Copy + std::ops::$Name<&'a S>,
    {   //
        fn [< $Name:snake >] (&mut self, rhs : &'a NumVec<S> )
        {   //
            if self.len() == 1 {
                if rhs.len() == 1 {
                    self.s $Op &(rhs.s);
                } else {
                    self.vec = vec![ self.s ; rhs.len() ];
                    for j in 0 .. rhs.len() {
                        self.vec[j] $Op &(rhs.vec[j]);
                    }
                }
            } else {
                if rhs.len() == 1 {
                    for j in 0 .. self.len() {
                        self.vec[j] $Op &(rhs.s);
                    }
                } else {
                    assert_eq!( self.len(), rhs.len() );
                    for j in 0 .. self.len() {
                        self.vec[j] $Op &(rhs.vec[j]);
                    }
                }
            }
        }
    }
} } }
//
num_vec_compound_op!(AddAssign, +=);
num_vec_compound_op!(SubAssign, -=);
num_vec_compound_op!(MulAssign, *=);
num_vec_compound_op!(DivAssign, /=);
// ----------------------------------------------------------------------------`
/// Displays a `NumVec` < *S* > object.
///
/// The text "[ " and " ]" surround the elements of the vector.
/// The elements are separated by ", "
/// and there is a "," after the last element.
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::NumVec;
/// let x     : Vec<f64>  = vec![5.0, 6.0];
/// let x_nv              = NumVec::new(x);
/// let s                 = format!( "{x_nv}" );
/// assert_eq!( s, "[ 5, 6, ]" );
/// ```
impl<S : std::fmt::Display> std::fmt::Display for NumVec<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")?;
        if self.len() == 1 {
            write!(f, "{}, ", self.s)?;
        } else {
            for j in 0 .. self.len() {
                write!(f, "{}, ", self.vec[j])?;
            }
        }
        write!(f, "]")
    }
}
// ----------------------------------------------------------------------------`
// From
/// Convert a scalar to a NumVec object with one element.
///
/// |           To             | From | From |
/// |--------------------------|------|------|
/// | `NumVec<f32>`            | f32  |      |
/// | `NumVec< AzFloat<f32> >` | f32  |      |
/// | `NumVec<f64>`            | f32  | f64  |
/// | `NumVec< AzFloat<f64> >` | f32  | f64  |
///
/// # Example
/// ```
/// use rustad::NumVec;
/// use rustad::AzFloat;
///
/// // f32 -> NumVec<f32>
/// let x                  = 3.0 as f32;
/// let x_nv : NumVec<f32> = NumVec::from(x);
/// assert_eq!( x_nv.len(), 1 );
/// assert_eq!( x_nv.get(0), 3.0 as f32 );
///
/// // f32 -> NumVec<f64>
/// let x                  = 3.0 as f32;
/// let x_nv : NumVec<f64> = NumVec::from(x);
/// assert_eq!( x_nv.len(), 1 );
/// assert_eq!( x_nv.get(0), 3.0 as f64 );
///
/// // f64 -> NumVec<f64>
/// let x                  = 3.0 as f64;
/// let x_nv : NumVec<f64> = NumVec::from(x);
/// assert_eq!( x_nv.len(), 1 );
/// assert_eq!( x_nv.get(0), 3.0 as f64 );
///
/// // f64 -> NumVec< AzFloat<f64> >
/// let x                  = 3.0 as f64;
/// let x_nv : NumVec< AzFloat<f64> > = NumVec::from(x);
/// assert_eq!( x_nv.len(), 1 );
/// assert_eq!( x_nv.get(0), AzFloat(3.0 as f64) );
/// ```
pub fn doc_from_scalar() {}
//
macro_rules! impl_from_scalar { ($F:ty, $T:ty ) => {
    #[doc = "see [doc_from_scalar]"]
    impl From<$F> for NumVec<$T> {
        fn from( scalar : $F )-> NumVec<$T> {
            NumVec { vec : Vec::new(), s : scalar.into() }
        }
    }
} }
impl_from_scalar!(f32, f32);
impl_from_scalar!(f32, f64);
impl_from_scalar!(f64, f64);
//
impl_from_scalar!(f32, AzFloat<f32>);
impl_from_scalar!(f32, AzFloat<f64>);
impl_from_scalar!(f64, AzFloat<f64>);
// ----------------------------------------------------------------------------`
// PartialEq
/// PartialEq `NumVec<S>` operator
///
/// S : is the type of the elements of the numeric vector.
///
/// Two NumVec object are equal it they have the same length
/// and their corresponding elements are equal.
/// The value Nan is considered equal to itself in the comparison.
///
/// Prototype:
/// <br/>
/// & `NumVec` < *S* > == & `NumVec` < *S* >
///
/// # Example
/// ```
/// use rustad::NumVec;
///
/// let a = NumVec::new( vec![ 2.0f64, 2.0f64 ] );
/// let b = NumVec::new( vec![ f64::NAN, 2.064 ] );
/// let c = NumVec::new( vec![ f64::NAN, 2.064 ] );
/// assert_eq!(a, a);
/// assert_eq!(b, b);
/// assert_eq!(c, c);
/// assert_ne!(a, b);
/// assert_eq!(b, c);
/// ```
pub fn doc_partial_equal() {}
///
macro_rules! impl_ordered_float_partial_equal { ($S:ident) => {
    impl PartialEq for NumVec<$S>
    where
        $S : PartialEq + ordered_float::FloatCore,
    {
        fn eq(&self, rhs : &Self) -> bool
        {   let mut result = true;
            if self.len() != rhs.len()  {
                result = false;
            } else if self.len() == 1 {
                result = OrderedFloat(self.s) == OrderedFloat(rhs.s);
            } else {
                for j in 0 .. self.len() {
                    result = result &&
                        OrderedFloat(self.vec[j]) == OrderedFloat(rhs.vec[j]);
                }
            }
            result
        }
    }
} }
impl_ordered_float_partial_equal!(f32);
impl_ordered_float_partial_equal!(f64);
///
macro_rules! impl_az_float_partial_equal { ($S:ident) => {
    impl PartialEq for NumVec< AzFloat<$S> >
    where
        AzFloat<$S> : PartialEq
    {
        fn eq(&self, rhs : &Self) -> bool
        {   let mut result = true;
            if self.len() != rhs.len()  {
                result = false;
            } else if self.len() == 1 {
                result = self.s == rhs.s;
            } else {
                for j in 0 .. self.len() {
                    result = result &&
                        self.vec[j] == rhs.vec[j];
                }
            }
            result
        }
    }
} }
impl_az_float_partial_equal!(f32);
impl_az_float_partial_equal!(f64);
