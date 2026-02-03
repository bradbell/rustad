// ============================================================================
// BEGIN az_float.rs
// ============================================================================
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
use super::cmp_as::{
    CmpAsLhs,
    CmpAsRhs,
};
use super::core::FloatCore;
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
///
/// * Zero : is an absolute zero; i.e. multiplication by zero
///   always results in zero (even if the other operand is nan).
///
/// *  Nan : is equal to Nan.
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
/// * Syntax : lhs op rhs
///
///     * lhs : is the AzFloat`<B>` left operand
///     * rhs : is the AzFloat`<B>` right operand
///     * op  : is one of `+` , `-` , `*` , `/`
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// type B = f64;
/// //
/// let z2 : AzFloat<B> = (2.0 as B).into();
/// let z4 : AzFloat<B> = (4.0 as B).into();
/// let z6 = z2 + z4;
/// assert_eq!( z6, (6.0 as B).into() );
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
    fn mul(self, rhs : Self) -> Self {
        let zero_b : B = 0f32.into();
        if (self.0) == zero_b || (rhs.0) == zero_b {
                Self( zero_b )
        } else {
            Self( (self.0).mul(rhs.0) )
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
        fn $name(self, rhs : Self) -> Self {
            Self( (self.0).$name(rhs.0) )
        }
    }
} }
impl_binary_operator!(Add, add);
impl_binary_operator!(Sub, sub);
impl_binary_operator!(Div, div);
// ---------------------------------------------------------------------------
// &AzFloat Op &AzFloat
/// AzFloat binary reference operations
///
/// * B : Is the floating point base type
///
/// * Syntax : &lhs op &rhs
///
///     * lhs : is the AzFloat`<B>` left operand
///     * rhs : is the AzFloat`<B>` right operand
///     * op  : is one of `+` , `-` , `*` , `/`
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// type B = f64;
/// //
/// let z6 : AzFloat<B> = (6.0 as B).into();
/// let z4 : AzFloat<B> = (4.0 as B).into();
/// let z2 = &z6 - &z4;
/// assert_eq!( z2, (2.0 as B).into() );
/// ```
pub fn doc_binary_reference() { }
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
macro_rules! impl_binary_reference{ ($Name:ident, $name:ident) => {
    #[doc = "see [doc_binary_reference]"]
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
impl_binary_reference!(Add, add);
impl_binary_reference!(Sub, sub);
impl_binary_reference!(Div, div);
// ---------------------------------------------------------------------------
// AzFloat Op &AzFloat
/// AzFloat binary assign operations
///
/// * B : Is the floating point base type
///
/// * Syntax : lhs op &rhs
///
///     * lhs : is the AzFloat`<B>` left operand
///     * rhs : is the AzFloat`<B>` right operand
///     * op  : is one of `+` , `-` , `*` , `/`
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// type B = f32;
/// //
/// let mut z12_4 : AzFloat<B> = (12.0 as B).into();
/// let z4        : AzFloat<B> = (4.0 as B).into();
/// z12_4         /= &z4;
/// assert_eq!( z12_4.to_inner(), (3.0 as B) );
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
} }
impl_binary_assign!(AddAssign, add_assign);
impl_binary_assign!(SubAssign, sub_assign);
impl_binary_assign!(DivAssign, div_assign);
// ---------------------------------------------------------------------------
// CmpAsLhs for AzFloat
/// CmpAsLhs when both operands are `AzFloat<B>`
///
/// * B : Is the floating point base type
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// use rustad::CmpAsLhs;
/// type B = f64;
/// //
/// let one   : AzFloat<B> = (1.0 as B).into();
/// let two   : AzFloat<B> = (2.0 as B).into();
/// let three : AzFloat<B> = (3.0 as B).into();
/// let lt     = two.left_lt(&three);
/// let not_lt = &one - &lt;
/// assert_eq!( lt.to_inner(), (1.0 as B) );
/// assert_eq!( not_lt.to_inner(), (0.0 as B) );
/// ```
pub fn doc_compare_left_az_float() {}
//
/// see [doc_compare_left_az_float]
macro_rules! impl_compare_left_az_float{ ($name:ident, $op:tt) => {
    #[doc = concat!( "CompareLeft trait for ", stringify!( $op ) ) ]
    fn $name(&self, other : & AzFloat<B> ) -> AzFloat<B> {
        let zero : AzFloat<B> = 0.into();
        let one  : AzFloat<B> = 1.into();
        //
        if self.0 $op other.0 {
            one
        } else {
            zero
        }
    }
} }
//
impl<B> CmpAsLhs for AzFloat<B>
where
    B          :  PartialOrd,
    AzFloat<B> : From<usize>,
{
    impl_compare_left_az_float!( left_lt, <  );
    impl_compare_left_az_float!( left_le, <= );
    impl_compare_left_az_float!( left_eq, == );
    impl_compare_left_az_float!( left_ne, != );
    impl_compare_left_az_float!( left_ge, >= );
    impl_compare_left_az_float!( left_gt, >  );
}
// ---------------------------------------------------------------------------
// CmpAsRhs for AzFloat
/// CmpAsRhs when both operands are `AzFloat<B>`
///
/// * B : Is the floating point base type
///
/// # Example :
/// ```
/// use rustad::AzFloat;
/// use rustad::CmpAsRhs;
/// type B = f64;
/// //
/// let one   : AzFloat<B> = (1.0 as B).into();
/// let two   : AzFloat<B> = (2.0 as B).into();
/// let three : AzFloat<B> = (3.0 as B).into();
/// let lt     = two.lt_right(&three);
/// let not_lt = &one - &lt;
/// assert_eq!( lt.to_inner(), (1.0 as B) );
/// assert_eq!( not_lt.to_inner(), (0.0 as B) );
/// ```
pub fn doc_compare_right_az_float() {}
//
/// see [doc_compare_right_az_float]
macro_rules! impl_compare_right_az_float{ ($name:ident, $op:tt) => {
    #[doc = concat!( "CmpAsRhs trait for ", stringify!( $op ) ) ]
    fn $name(&self, other : & AzFloat<B> ) -> AzFloat<B> {
        let zero : AzFloat<B> = 0.into();
        let one  : AzFloat<B> = 1.into();
        //
        if self.0 $op other.0 {
            one
        } else {
            zero
        }
    }
} }
//
impl<B> CmpAsRhs for AzFloat<B>
where
    B          :  PartialOrd,
    AzFloat<B> : From<usize>,
{
    impl_compare_right_az_float!( lt_right, <  );
    impl_compare_right_az_float!( le_right, <= );
    impl_compare_right_az_float!( eq_right, == );
    impl_compare_right_az_float!( ne_right, != );
    impl_compare_right_az_float!( ge_right, >= );
    impl_compare_right_az_float!( gt_right, >  );
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
// ----------------------------------------------------------------------------
/// FloatCore trait for az_float types
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_float_core{ ($P:ident) => {
    impl FloatCore for crate::AzFloat<$P> {
        fn nan()  -> Self { Self( $P::NAN ) }
        fn zero() -> Self { Self( 0 as $P ) }
        fn one()  -> Self { Self( 1 as $P ) }
    }
}}
impl_float_core!(f32);
impl_float_core!(f64);
