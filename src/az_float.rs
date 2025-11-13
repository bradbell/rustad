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
/// B : the floating point base class is either f32 or f64
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
    /// Determine if the floating point base is nan for this object
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
//
// From<f32>
/// Converterst from f32 to an AzFloat object
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
/// Converterst from f64 to an AzFloat object
impl From<f64> for AzFloat<f64>
{
    fn from(f : f64) -> Self {
        Self( f.into()  )
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
macro_rules! impl_binary_operator{ ($Name:ident) => { paste::paste! {
    #[doc = "see [doc_binary_operator]"]
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
macro_rules! impl_binary_reference{ ($Name:ident) => { paste::paste! {
    #[doc = "see [doc_binary_reference]"]
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
macro_rules! impl_binary_assign{ ($Name:ident) => { paste::paste! {
    #[doc = "see [doc_binary_assign]"]
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
/// use std::collections::HashMap;
/// use rustad::AzFloat;
///
/// let mut map : HashMap<AzFloat<f32>, u32> = HashMap::new();
/// let z1      = AzFloat(1f32);
/// let z2      = AzFloat(2f32);
/// let z3      = AzFloat( f32::NAN );
/// map.insert(z1, 1u32);
/// map.insert(z2, 2u32);
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
    {
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

#[cfg(test)]
mod tests {
    use crate::{
        AzFloat,
        start_recording_dyp,
        stop_recording,
        ad_from_value,
    };
    //
    #[test]
    fn az_forward_dyp() {
        //
        // V
        type V = AzFloat<f32>;
        //
        // np, nx, p, x
        let np         = 3;
        let nx         = 1;
        let p : Vec<V> = vec![ V::from(1.0) ; np ];
        let x : Vec<V> = vec![ V::from(1.0) ; nx ];
        //
        // asum
        // The first addition adds the constants zero and so is not recorded
        let (ap, ax)   = start_recording_dyp(p.clone(), x.clone());
        let mut asum   = ad_from_value( V::from(0.0) );
        for j in 0 .. np {
            asum += &ap[j];
        }
        //
        // f
        let ay = vec![ &ax[0] * &asum ];
        let f  = stop_recording(ay);
        //
        // dyp_both
        let trace = true;
        let dyp_both = f.forward_dyp_value(p.clone(), trace);
        //
        assert_eq!( dyp_both.len(), 2 * np - 1 );
        for j in 0 .. np {
            assert_eq!( dyp_both[j], p[j] );
        }
        let mut sum = p[0];
        for j in 1 .. np {
        sum += &p[j];
            assert_eq!( dyp_both[np + j - 1], sum );
        }
    }
}
