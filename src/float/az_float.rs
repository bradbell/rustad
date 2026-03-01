// ---------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the rustad AzFloat class.
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust and src/float.
//! This enables src/float to be directly included as part of a Dll library.
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
use crate::{
    FloatCore,
    NumCmp,
    Powf,
};
// ---------------------------------------------------------------------------
/// The Absolute Zero Floating point class.
///
/// * Motivation :
///
///     * Forward Mode :
///       During forward mode AD, the partial derivaitve of f(x) w.r.t x_i
///       is evaluated as f'(x) * e
///       where e_j is one (zero) if j is equal to i (not equal to i).
///       If zero times nan were nan, and one of the elements of f'(x)
///       were nan, the partial of f w.r.t. x_i would evaluate to nan
///       (even if the corresponding column of f'(x) did not have a nan).
///
///     * Reverse Mode :
///       A similar effect to forward mode is present in reverse mode.
///
///     * Optimization :
///       Because zero times anything (including a variable)
///       is the constant zero, multiplications by the
///       constant zero does not need to be recorded in the tape.
///
///     * Conditional Expressions :
///       Comparison operators; e.g. <=, are represented by 1 for
///       true and 0 for false. It follows that the two expressions
///       below are equivalent :
///       ```text
///           if x <= y { u } else {v}
///           x.le(y) * u + x.gt(y) * v
///       ```
///       Note that this works element wise when x, y, u, v
///       are numeric vectors.
///
/// * B : the floating point base class is either f32 or f64
///
/// * Zero : is an absolute zero; i.e. multiplication by zero
///   always results in zero (even if the other operand is nan).
///
/// * Nan : is equal to Nan.
///
/// * Copy : The Copy trait is implemented for these types.
///
/// # Example
/// ```
/// use rustad::AzFloat;
/// //
/// let zero  = AzFloat( 0f32 );
/// let nan   = AzFloat( f32::NAN );
/// let prod  = zero * nan;
/// assert_eq!( prod, zero );
/// assert_eq!( nan , nan );
///
#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct AzFloat<B>(pub B);
//
impl<B> AzFloat<B>
where
    B : PartialEq ,
{
    //
    // is_nan
    /// Determine if the floating point base is nan for this object
    #[allow(clippy::eq_op)]
    pub fn is_nan(&self) -> bool {
        self.0 != self.0
    }
    //
    // to_inner
    /// Returns the floating point base for this object
    pub fn to_inner(self) -> B {
        self.0
    }
}
// ---------------------------------------------------------------------------
/// AzFloat From
///
/// * `AzFloat<f32>` :
///   From is implemented for usize and f32 .
///
/// * `AzFloat<f64>` :
///   From is implemented for :usize, f32, f64, and `AzFloat<f32>` .
///
pub fn doc_impl_from() {}
//
macro_rules! impl_from_primitive{ ($P:ident, $T:ident) => {
    #[doc = "see [doc_impl_from]" ]
    impl From<$P> for AzFloat<$T> {
        fn from(p : $P) -> AzFloat<$T> {
            AzFloat( p as $T )
        }
    }
} }
impl_from_primitive!(usize, f32);
impl_from_primitive!(f32, f32);
impl_from_primitive!(usize, f64);
impl_from_primitive!(f32, f64);
impl_from_primitive!(f64, f64);
//
/// see [doc_impl_from]
impl From< AzFloat<f32> > for AzFloat<f64> {
    fn from( z : AzFloat<f32> ) -> AzFloat<f64> {
        AzFloat( z.0 as f64 )
    }
}
// ---------------------------------------------------------------------------
// AzFloat Op AzFloat
/// AzFloat binary operations
///
/// * B : Is the floating point base type
///
/// * Syntax :
/// ```text
///     z = x Op y
/// ```
///
/// * Op : is the source code token for this binary operator;
///   i.e., `+` , `-` , `*` , or `/` .
///
/// * x : left hand side `AzFloat<V>` or `&AzFloat<V>` object
/// * y : left hand side `AzFloat<V>` or `&AzFloat<V>` object
/// * z : result `AzFloat<V>` object
///
/// If the left or right operand is borrowed (&), then both operands
/// must be borrowed.
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// type V = AzFloat<f64>;
/// //
/// let x  = V::from(2.0);
/// let y  = V::from(4.0);
/// let z  = x + y;
/// assert_eq!( z, V::from(6.0) );
/// let z  = &x * &y;
/// assert_eq!( z, V::from(8.0) );
/// ```
pub fn doc_binary_operator() { }
///
//
// Mul
/// see [doc_binary_operator]
impl<B> Mul for AzFloat<B>
where
    B : From<f32> + PartialEq + Mul<Output=B>,
{
    type Output = AzFloat<B>;
    fn mul(self, rhs : Self) -> AzFloat<B> {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                Self( zero_b )
        } else {
            Self( (self.0).mul(rhs.0) )
        }
    }
}
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
macro_rules! impl_binary_operator{ ($Name:ident, $name:ident) =>  {
    #[doc = "see [doc_binary_operator]"]
    impl<B> $Name for AzFloat<B>
    where
        B : From<f32> + PartialEq + $Name<Output=B>,
    {
        type Output = AzFloat<B>;
        fn $name(self, rhs : Self) -> AzFloat<B> {
            Self( (self.0).$name(rhs.0) )
        }
    }
    #[doc = "see [doc_binary_operator]"]
    impl<B> $Name<& AzFloat<B> > for &AzFloat<B>
    where
        for<'a> &'a B : $Name<&'a B, Output=B>,
        B : From<f32> + PartialEq ,
    {
        type Output = AzFloat<B>;
        fn $name (self, rhs : & AzFloat<B> ) -> AzFloat<B> {
            AzFloat( (self.0).$name(&rhs.0) )
        }
    }
} }
impl_binary_operator!(Add, add);
impl_binary_operator!(Sub, sub);
impl_binary_operator!(Div, div);
// ---------------------------------------------------------------------------
// AzFloat Op &AzFloat
/// AzFloat binary assign operations
///
/// * B : Is the floating point base type
///
/// * Syntax :
///   ```text
///     lhs op rhs
///     lhs op &rhs
///   ```
///
/// * lhs : is the `AzFloat<B>` left operand
/// * rhs : is the `AzFloat<B>` right operand
/// * op  : is one of `+=` , `-=` , `*=` , `/=`
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// type V = AzFloat<f32>;
/// //
/// let mut x = V::from(12.0);
/// let y     = V::from(4.0);
/// //
/// x        /= &y;
/// assert_eq!( x, V::from(3.0) );
/// //
/// x        -= y;
/// assert_eq!( x, V::from(-1.0) );
/// ```
pub fn doc_binary_assign() {}
//
/// see [doc_binary_assign]
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
//
/// see [doc_binary_assign]
impl<B> MulAssign<AzFloat<B> > for AzFloat<B>
where
    B : From<f32> + PartialEq + for<'a> MulAssign<&'a B>,
{
    fn mul_assign(&mut self, rhs : AzFloat<B>) {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                self.0 = zero_b;
        } else {
            self.0.mul_assign( &rhs.0 );
        }
    }
}
macro_rules! impl_binary_assign{ ($Name:ident, $name:ident) => {
    #[doc = "see [doc_binary_assign]"]
    impl<B> $Name <& AzFloat<B> > for AzFloat<B>
    where
        B : From<f32> + PartialEq +  for<'a> $Name <&'a B>,
    {
        fn $name(& mut self, rhs : & AzFloat<B> ) {
            self.0.$name(&rhs.0);
        }
    }
    #[doc = "see [doc_binary_assign]"]
    impl<B> $Name <AzFloat<B> > for AzFloat<B>
    where
        B : From<f32> + PartialEq +  for<'a> $Name <&'a B>,
    {
        fn $name(& mut self, rhs : AzFloat<B> ) {
            self.0.$name(&rhs.0);
        }
    }
} }
impl_binary_assign!(AddAssign, add_assign);
impl_binary_assign!(SubAssign, sub_assign);
impl_binary_assign!(DivAssign, div_assign);
// ---------------------------------------------------------------------------
// NumCmp for AzFloat
/// Implement [NumCmp] when both operands are `AzFloat<B>` or `&AzFloat<B>` .
///
/// * B : Is the floating point base type; see [AzFloat]
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// use rustad::NumCmp;
/// type V = AzFloat<f64>;
/// //
/// let zero  = V::from(0);
/// let one   = V::from(1);
/// let two   = V::from(2);
/// let three = V::from(3);
/// let lt     = two.num_lt(three);
/// let not_lt = one - lt;
/// assert_eq!(lt, one);
/// assert_eq!(not_lt, zero);
/// ```
pub fn doc_num_cmp_az_float() {}
//
/// see [doc_num_cmp_az_float]
macro_rules! impl_num_cmp_az_float_borrow{ ($name:ident, $op:tt) => {
    #[doc = concat!( " AzFloat::", stringify!($name)  ) ]
    fn $name(self, rhs : & AzFloat<B> ) -> AzFloat<B> {
        let zero : AzFloat<B> = FloatCore::zero();
        let one  : AzFloat<B> = FloatCore::one();
        //
        if self.0 $op rhs.0 {
            one
        } else {
            zero
        }
    }
} }
//
impl<B> NumCmp< &AzFloat<B> > for &AzFloat<B>
where
    B          : PartialOrd,
    AzFloat<B> : FloatCore,
{
    type Output = AzFloat<B>;
    //
    impl_num_cmp_az_float_borrow!( num_lt, <  );
    impl_num_cmp_az_float_borrow!( num_le, <= );
    impl_num_cmp_az_float_borrow!( num_eq, == );
    impl_num_cmp_az_float_borrow!( num_ne, != );
    impl_num_cmp_az_float_borrow!( num_ge, >= );
    impl_num_cmp_az_float_borrow!( num_gt, >  );
}
//
/// see [doc_num_cmp_az_float]
macro_rules! impl_num_cmp_az_float_own{ ($name:ident) => {
    #[doc = concat!( " AzFloat::", stringify!($name)  ) ]
    fn $name(self : AzFloat<B>, rhs : AzFloat<B>) -> AzFloat<B> {
        NumCmp::$name( &self,  &rhs )
    }
} }
//
impl<B> NumCmp< AzFloat<B> > for AzFloat<B>
where
    B          : PartialOrd,
    AzFloat<B> : FloatCore,
    for<'a> &'a AzFloat<B> : NumCmp< &'a AzFloat<B>, Output = AzFloat<B> >,
{
    type Output = AzFloat<B>;
    //
    impl_num_cmp_az_float_own!( num_lt );
    impl_num_cmp_az_float_own!( num_le );
    impl_num_cmp_az_float_own!( num_eq );
    impl_num_cmp_az_float_own!( num_ne );
    impl_num_cmp_az_float_own!( num_ge );
    impl_num_cmp_az_float_own!( num_gt );
}
// ---------------------------------------------------------------------------
// PartialEq, Eq
/// AzFloat Eq Operator
///
/// * B : is the floating point base type
///
/// # Example
/// ```
/// use rustad::AzFloat;
/// let nan   = AzFloat( f32::NAN );
/// assert_eq!( nan, nan );
/// ```
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
// ---------------------------------------------------------------------------
/// Display the an AzFloat object
///
/// * B : is the floating point base type
///
/// # Example
/// ```
/// use rustad::AzFloat;
/// let nan = AzFloat( f64::NAN );
/// let s   = format!( "{nan}" );
/// assert_eq!(s, "NaN");
/// ```
impl<B> std::fmt::Display for AzFloat<B>
where
    B : std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// ---------------------------------------------------------------------------
/// Hash function for AzFloat objects
///
/// * B : is the floating point base type
///
/// # Example
/// ```
/// use rustc_hash::FxHashMap;
/// use rustad::AzFloat;
///
/// let mut map : FxHashMap<AzFloat<f32>, u32> = FxHashMap::default();
/// let z1      = AzFloat(1f32);
/// let z2      = AzFloat(2f32);
/// let z3      = AzFloat( f32::NAN );
/// map.insert(z1, 1u32);
/// map.insert(z2, 2u32);
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
/// map.insert(z3, 3u32);
/// let option  = map.get_key_value(&z3);
/// assert_eq!(option, Some( (&z3, &3u32) ) );
///
/// ```
pub fn doc_hash_trait() {}
//
macro_rules! impl_hash_trait{ ($B:ident) => {
    /// see [doc_hash_trait]
    impl std::hash::Hash for AzFloat<$B>
    {   // see [doc_hash_trait]
        fn hash<H : std::hash::Hasher>(&self, state : &mut H) {
            let bits       = self.0.to_bits();
            let uint : u64 = bits.into();
            uint.hash(state);
        }
    }
} }
impl_hash_trait!(f32);
impl_hash_trait!(f64);
// ---------------------------------------------------------------------------
/// powf function for AzFloat objects
///
/// * B : is the floating point base type
///
/// # Example
/// ```
/// use rustad::{
///     AzFloat,
///     Powf,
/// };
/// let two      = AzFloat(2f32);
/// let three    = AzFloat(3f32);
/// let eight    = AzFloat(8f32);
/// let powf_23  = two.powf(three);
/// assert_eq!(powf_23, eight);
/// let powf_23  = (&two).powf(&three);
/// assert_eq!(powf_23, eight);
/// ```
pub fn doc_powf_az_float() {}
//
macro_rules! impl_powf_trait{ ($B:ident) => {
    impl Powf for AzFloat<$B> {
        type Output = AzFloat<$B>;
        //
        // see [doc_powf_az_float]
        fn powf(self, rhs : AzFloat<$B>) -> AzFloat<$B> {
            AzFloat( self.0.powf( rhs.0 ) )
        }
    }
    impl Powf for &AzFloat<$B> {
        type Output = AzFloat<$B>;
        //
        // see [doc_powf_az_float]
        fn powf(self, rhs : &AzFloat<$B>) -> AzFloat<$B> {
            AzFloat( self.0.powf( rhs.0 ) )
        }
    }
} }
impl_powf_trait!(f32);
impl_powf_trait!(f64);
// ----------------------------------------------------------------------------
macro_rules! float_core_unary_function{ ($B:ident, $name:ident) => {
    #[doc = concat!(
        "`AzFloat<", stringify!($B), ">`.", stringify!($name), "()"
    )]
    fn $name(&self) -> AzFloat<$B> { Self( self.0.$name() ) }
} }
/// FloatCore trait for az_float types
///
/// * B : is the floating point base type
macro_rules! impl_float_core{ ($B:ident) => {
    impl FloatCore for AzFloat<$B> {
        fn pi()           -> AzFloat<$B> { Self( std::$B::consts::PI ) }
        fn nan()          -> AzFloat<$B> { Self( $B::NAN ) }
        fn one()          -> AzFloat<$B> { Self( 1 as $B ) }
        fn zero()         -> AzFloat<$B> { Self( 0 as $B ) }
        fn epsilon()      -> AzFloat<$B> { Self( $B::EPSILON ) }
        fn min_positive() -> AzFloat<$B> { Self( $B::MIN_POSITIVE ) }
        //
        // unary functions
        float_core_unary_function!($B, ln);
        float_core_unary_function!($B, sqrt);
        float_core_unary_function!($B, tanh);
        float_core_unary_function!($B, tan);
        float_core_unary_function!($B, sinh);
        float_core_unary_function!($B, cosh);
        float_core_unary_function!($B, abs);
        float_core_unary_function!($B, signum);
        float_core_unary_function!($B, exp);
        float_core_unary_function!($B, cos);
        float_core_unary_function!($B, sin);
        //
        // unary function that implements differently
        #[doc = concat!( "`AzFloat<", stringify!($B), ">`.minus()" )]
        fn minus(&self) -> AzFloat<$B> { Self( - self.0 ) }
        //
        // binary functions
        fn powi(&self, rhs : i32) -> AzFloat<$B>{ Self( self.0.powi(rhs) ) }
    }
} }
impl_float_core!(f32);
impl_float_core!(f64);
