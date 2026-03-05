// ---------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the numeric vector class `NumVec` < *S* >.
//!
//! Link to [parent module](super)
//!
//! This module does not have any dependencies outside the standard rust
//! and az_float.
//! This enables it source code to be directly included as part of a
//! Dll library (This has not yet been tested).
// ---------------------------------------------------------------------------
// use
use crate::{
    AzFloat,
    Powf,
    NumCmp,
    FConst,
    FUnary,
};
//
// NumVec
/// The numeric vector class.
///
/// * S :
///   is the scalar type; i.e., the type of elements of the vector.
///
/// * Scalars :
///   If a NunVec has one element, it acts like a scalar; i.e.,
///   a vector with any required length and all the element
///   equal to the scalar value.
///
/// * Copy, Clone :
///   The NumVec types implement Clone, but not the Copy trait.
///
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
    pub fn new( v : Vec<S> ) -> NumVec<S> {
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
        if self.vec.is_empty() {
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
/// type S = f64;
/// let a  = NumVec::new( vec![ S::from(1.0), S::from(2.0) ] );
/// let b  = NumVec::from( S::from(3.0) );
/// let c  = NumVec::from( S::from(4.0) );
///
/// let d = &a * &b;
/// assert_eq!( d.len(), 2);
/// assert_eq!( d.get(0), S::from(3.0) );
/// assert_eq!( d.get(1), S::from(6.0) );
///
/// let d = &b - &c;
/// assert_eq!( d.len(), 1);
/// assert_eq!( d.get(0), S::from(-1.0) );
/// ```
pub fn doc_num_vec_binary_op() { }
//
/// Add one binary operator to the `NumVec` < *S* > class;
/// see [doc_num_vec_binary_op]
macro_rules! num_vec_binary_op { ($Name:ident, $name:ident, $Op:tt) => {

    #[doc = concat!(
        "& `NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_num_vec_binary_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for &'a NumVec<S>
    where
        S : From<f32> + Copy + std::ops::$Name<Output=S> ,
    {   type Output = NumVec<S>;
        //
        fn $name(self : &'a NumVec<S>, rhs : &'a NumVec<S> )
        -> NumVec<S>
        {   let mut v : Vec<S>;
            let e     : S;
            if self.len() == 1 {
                if rhs.len() == 1 {
                    e = self.s $Op rhs.s;
                    v = Vec::new();
                } else {
                    e = f32::NAN.into();
                    v = Vec::with_capacity( rhs.len() );
                    for j in 0 .. rhs.len() { v.push(
                        self.s $Op rhs.vec[j]
                    ); }
                }
            } else {
                e = f32::NAN.into();
                v = Vec::with_capacity( self.len() );
                if rhs.len() == 1 {
                    for j in 0 .. self.len() { v.push(
                        self.vec[j] $Op rhs.s
                    ); }
                } else {
                    assert_eq!( self.len(), rhs.len() );
                    for j in 0 .. self.len() { v.push(
                        self.vec[j] $Op rhs.vec[j]
                    ); }
                }
            }
            NumVec{ vec : v, s : e }
        }
    }
} }
//
num_vec_binary_op!(Add, add, +);
num_vec_binary_op!(Sub, sub,  -);
num_vec_binary_op!(Mul, mul, *);
num_vec_binary_op!(Div, div, /);
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
/// type S    = f32;
/// let mut a = NumVec::new( vec![ S::from(12.0), S::from(6.0) ] );
/// let mut b = NumVec::from( S::from(3.0) );
/// let c     = NumVec::from( S::from(4.0) );
///
/// a /= &b;
/// assert_eq!( a.len(), 2);
/// assert_eq!( a.get(0), S::from(4.0) );
/// assert_eq!( a.get(1), S::from(2.0) );
///
/// b += &c;
/// assert_eq!( b.len(), 1);
/// assert_eq!( b.get(0), S::from(7.0) );
/// ```
pub fn doc_num_vec_compound_op() { }
//
/// Add one compound assignment operator to the `NumVec` < *S* > class;
/// see [doc_num_vec_compound_op]
macro_rules! num_vec_compound_op { ($Name:ident, $name:ident, $Op:tt) => {

    #[doc = concat!(
        "`NumVec` < *S* > ", stringify!($Op), " & `NumVec` < *S* >",
        "; see [doc_num_vec_compound_op]"
    )]
    impl<'a, S> std::ops::$Name< &'a NumVec<S> > for NumVec<S>
    where
        S : Copy + std::ops::$Name<&'a S>,
    {   //
        fn $name(&mut self, rhs : &'a NumVec<S> )
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
} }
//
num_vec_compound_op!(AddAssign, add_assign, +=);
num_vec_compound_op!(SubAssign, sub_assign, -=);
num_vec_compound_op!(MulAssign, mul_assign, *=);
num_vec_compound_op!(DivAssign, div_assign, /=);
// ---------------------------------------------------------------------------
// NumCmp for NumVec
/// Implement [NumCmp] when both operands are `NumVec<S>` or `&NumVec<S>` .
///
/// * S : is the type of the elements of the numeric vector.
///
/// Note that these functions act element-wise on each `NumVec<S>` object.
///
/// # Example :
/// ```
/// use rustad::{
///     NumVec,
///     AzFloat,
///     NumCmp,
/// };
///
/// type S = AzFloat<f32>;
/// //
/// let two_three = NumVec::new( vec![ S::from(2), S::from(3) ] );
/// let three     = NumVec::from( S::from(3) );
///
/// let lt        = NumCmp::num_lt( &two_three, &three );
/// let check     = NumVec::new( vec![ S::from(1), S::from(0) ] );
/// assert_eq!( lt, check);
///
/// let one       = NumVec::from( S::from(1) );
/// let not_lt    = &one - &lt;
/// let check     = NumVec::new( vec![ S::from(0), S::from(1) ] );
/// assert_eq!(not_lt, check);
/// ```
pub fn doc_num_cmp_num_vec() {}
//
/// see [doc_num_cmp_num_vec]
macro_rules! impl_num_cmp_num_vec_borrow{ ($name:ident, $op:tt) => {
    #[doc = concat!( "NumVec::", stringify!( $name ) ) ]
    fn $name(self, rhs : & NumVec<S> ) -> NumVec<S> {
        //
        let mut v : Vec<S>;
        let e     : S;
        //
        if self.len() == 1 {
            if rhs.len() == 1 {
                e = self.s.$name( &rhs.s );
                v = Vec::new();
            } else {
                e = FConst::nan();
                v = Vec::with_capacity( rhs.len() );
                for j in 0 .. rhs.len() { v.push(
                    self.s.$name(&rhs.vec[j])
                ); }
            }
        } else {
            e = FConst::nan();
            v = Vec::with_capacity( self.len() );
            if rhs.len() == 1 {
                for j in 0 .. self.len() { v.push(
                    self.vec[j].$name(&rhs.s )
                ); }
            } else {
                assert_eq!( self.len(), rhs.len() );
                for j in 0 .. self.len() { v.push(
                    self.vec[j].$name(& rhs.vec[j] )
                ); }
            }
        }
        NumVec{ vec : v, s : e }
    }
} }
//
impl<S> NumCmp< &NumVec<S> > for &NumVec<S>
where
    S              : FConst,
    for<'a> &'a S  : NumCmp<&'a S, Output = S>,
{
    type Output = NumVec<S>;
    //
    impl_num_cmp_num_vec_borrow!( num_lt, <  );
    impl_num_cmp_num_vec_borrow!( num_le, <= );
    impl_num_cmp_num_vec_borrow!( num_eq, == );
    impl_num_cmp_num_vec_borrow!( num_ne, != );
    impl_num_cmp_num_vec_borrow!( num_ge, >= );
    impl_num_cmp_num_vec_borrow!( num_gt, >  );
}
//
/// see [doc_num_cmp_num_vec]
macro_rules! impl_num_cmp_num_vec_own{ ($name:ident) => {
    #[doc = concat!( " NumVec::", stringify!($name)  ) ]
    fn $name(self : NumVec<S>, rhs : NumVec<S>) -> NumVec<S> {
        NumCmp::$name( &self,  &rhs )
    }
} }
//
impl<S> NumCmp< NumVec<S> > for NumVec<S>
where
    S          : PartialOrd,
    NumVec<S> : FConst,
    for<'a> &'a NumVec<S> : NumCmp< &'a NumVec<S>, Output = NumVec<S> >,
{
    type Output = NumVec<S>;
    //
    impl_num_cmp_num_vec_own!( num_lt );
    impl_num_cmp_num_vec_own!( num_le );
    impl_num_cmp_num_vec_own!( num_eq );
    impl_num_cmp_num_vec_own!( num_ne );
    impl_num_cmp_num_vec_own!( num_ge );
    impl_num_cmp_num_vec_own!( num_gt );
}
//
// ----------------------------------------------------------------------------`
/// Displays a `NumVec` < *S* > object.
///
/// The text "[ " and " ]" surround the elements of the vector.
/// The elements are separated by ", "
/// and there is a "," after the last element.
///
/// # Example using NumVec
/// ```
/// use rustad::NumVec;
/// type S    = f64;
/// let x     = vec![ S::from(5.0) , S::from(6.0) ];
/// let x_nv  = NumVec::new(x);
/// let s     = format!( "{x_nv}" );
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
//
impl_from_scalar!( AzFloat<f32>, AzFloat<f32>);
impl_from_scalar!( AzFloat<f32>, AzFloat<f64>);
impl_from_scalar!( AzFloat<f64>, AzFloat<f64>);
// ----------------------------------------------------------------------------`
// PartialEq, Eq
/// `NumVec<S>` Eq operator
///
/// * S : is the type of the elements of the numeric vector.
///
/// Two NumVec object are equal it they have the same length
/// and their corresponding elements are equal.
///
/// Prototype:
/// <br/>
/// & `NumVec` < *S* > == & `NumVec` < *S* >
///
/// # Example
/// ```
/// use rustad::NumVec;
/// use rustad::AzFloat;
///
/// type S = AzFloat<f32>;
/// let a  = NumVec::new( vec![ S::from(f32::NAN), S::from(2.0) ] );
/// let b  = NumVec::new( vec![ S::from(2.0), S::from(2.0) ] );
/// let c  = NumVec::from( S::from(2.0) );
/// assert_eq!(a, a);
/// assert_eq!(b, b);
/// assert_eq!(c, c);
///
/// assert_ne!(a, b);
/// assert_ne!(a, c);
/// assert_ne!(b, c);
/// ```
pub fn doc_partial_equal() {}
//
impl<S> PartialEq for NumVec<S>
where
    S : PartialEq,
{
    fn eq(&self, rhs : &Self) -> bool
    {   if self.len() != rhs.len()  {
            false
        } else if self.len() == 1 {
            self.s == rhs.s
        } else {
            self.vec == rhs.vec
        }
    }
}
impl<S: PartialEq> Eq for NumVec<S> { }
// ---------------------------------------------------------------------------
/// Hash function for `NumVec<AzFloat>` objects
///
/// * S : is the type of the elements of the numeric vector.
///
/// # Example
/// ```
/// use rustc_hash::FxHashMap;
/// use rustad::AzFloat;
/// use rustad::NumVec;
///
/// type S      = AzFloat<f64>;
/// type V      = NumVec<S>;
/// let mut map : FxHashMap<V, u32> = FxHashMap::default();
/// let z1      = NumVec::new( vec![ S::from(1.0) ] );
/// let z2      = NumVec::new( vec![ S::from(1.0), S::from( f64::NAN) ] );
/// let z3      = NumVec::new( vec![ S::from( f64::NAN ) ]  );
/// map.insert(z1.clone(), 1u32);
/// map.insert(z2.clone(), 2u32);
///
/// let option  = map.get_key_value(&z1);
/// assert_eq!(option, Some( (&z1, &1u32) ) );
///
/// let option  = map.get_key_value(&z2);
/// assert_eq!(option, Some( (&z2, &2u32) ) );
///
/// let option  = map.get_key_value(&z3);
/// assert_eq!(option, None );
///
/// map.insert(z3.clone(), 3u32);
/// let option  = map.get_key_value(&z3);
/// assert_eq!(option, Some( (&z3, &3u32) ) );
///
/// ```
impl<S> std::hash::Hash for NumVec<S>
where
    S : std::hash::Hash,
{
    fn hash<H : std::hash::Hasher>(&self, state : &mut H) {
        if self.len() == 1 {
            self.s.hash(state);
        } else {
            self.vec.hash(state);
        }
    }
 }
// ---------------------------------------------------------------------------
/// powf function for NumVec objects
///
/// * S : is the floating point scalar type
///
/// # Example
/// ```
/// use rustad::{
///     AzFloat,
///     NumVec,
///     Powf,
/// };
/// let two      = NumVec::new( vec![ AzFloat(2f32) ] );
/// let three    = NumVec::new( vec![ AzFloat(3f32) ] );
/// let eight    = NumVec::new( vec![ AzFloat(8f32) ] );
/// let powf_23  = Powf::powf( &two, &three);
/// assert_eq!(powf_23, eight);
/// ```
pub fn doc_powf_num_vec() {}
//
impl<S> Powf< &NumVec<S> > for &NumVec<S>
where
    S : FConst + Copy + Powf<Output=S> ,
{   //
    type Output = NumVec<S>;
    //
    #[doc = "see [doc_powf_num_vec]" ]
    fn powf(self, rhs : &NumVec<S>) -> NumVec<S> {
        if self.len() == 1 {
            if rhs.len() == 1 {
                NumVec::<S> { s : self.s.powf( rhs.s ), vec : Vec::new() }
            } else {
                let v = rhs.vec.iter().map( |s| self.s.powf(*s) ).collect();
                NumVec::<S> { s : FConst::nan(), vec : v }
            }
        } else if rhs.len() == 1 {
            let v = self.vec.iter().map( |s| s.powf( rhs.s) ).collect();
            NumVec::<S> { s : FConst::nan() , vec : v }
        } else {
            assert_eq!( self.len(), rhs.len() );
            let mut v : Vec<S> = Vec::with_capacity( self.len() );
            for j in 0 .. self.len() {
                v.push( self.vec[j].powf( rhs.vec[j] ) );
            }
            NumVec::<S> { s : FConst::nan() , vec : v }
        }
    }
}
// ---------------------------------------------------------------------------
/// Implements the FConst trait for NumVec types
impl<S> FConst for NumVec<S>
where
    S         : FConst + From<f32>,
    NumVec<S> : From<S>
{
    fn pi()           -> NumVec<S> { Self::from( S::pi() ) }
    fn nan()          -> NumVec<S> { Self::from( S::nan() ) }
    fn one()          -> NumVec<S> { Self::from( S::one() ) }
    fn zero()         -> NumVec<S> { Self::from( S::zero() ) }
    fn epsilon()      -> NumVec<S> { Self::from( S::epsilon() ) }
    fn min_positive() -> NumVec<S> { Self::from( S::min_positive() ) }
}
// ---------------------------------------------------------------------------
macro_rules! float_unary_function{ ($name:ident) => {
    #[ doc = concat!( "`NumVec<S>.`", stringify!($name), "()" )]
    fn $name(self) -> NumVec<S> {
        if self.len() == 1 {
            NumVec { s : self.s.$name() , vec : Vec::new() }
        } else {
            let v = self.vec.iter().map( |s| s.$name() ).collect();
            NumVec { s : f32::NAN.into() , vec : v }
        }
    }
} }
/// Implements the FUnary trait for &NumVec types
impl<S> FUnary for &NumVec<S>
where
    S             : From<f32>,
    for<'a> &'a S : FUnary<Output=S>,
    NumVec<S>     : From<S>
{
    type Output = NumVec<S>;
    //
    // use float_unary_function
    float_unary_function!(exp_m1);
    float_unary_function!(ln);
    float_unary_function!(sqrt);
    float_unary_function!(tanh);
    float_unary_function!(tan);
    float_unary_function!(sinh);
    float_unary_function!(cosh);
    float_unary_function!(abs);
    float_unary_function!(signum);
    float_unary_function!(exp);
    float_unary_function!(minus);
    float_unary_function!(cos);
    float_unary_function!(sin);
    //
    // powi
    /// `NumVec<S>`.powi(`i32`)
    fn powi(self, rhs : i32) -> NumVec<S> {
        if self.len() == 1 {
            NumVec { s : self.s.powi(rhs) , vec : Vec::new() }
        } else {
            let v = self.vec.iter().map( |s| s.powi(rhs) ).collect();
            NumVec { s : f32::NAN.into() , vec : v }
        }
    }
}
