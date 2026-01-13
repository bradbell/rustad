// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the automatic differentiation class `AD<V>`.
//!
//! Link to [parent module](super)
//!
//
// ---------------------------------------------------------------------------
// sub-modules
//
pub mod binary;
pub mod compare;
// ---------------------------------------------------------------------------
// use
//
use crate::AzFloat;
// ---------------------------------------------------------------------------
//
// ADType
/// The AD types satisfy the following order:
/// constants < dynamic parameters < variables.
///
/// If a result depends on two arguments, the type of the result is the
/// maximum of the type of its arguments.
/// The value Empty is greater than any other type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ADType {
    //
    // ConstantP
    /// An AD object is a constant parameter if
    /// it does not depend on the value of
    /// the domain variables or domain dynamic parameters.
    ConstantP,
    //
    // DynamicP
    /// An AD object is a dynamic parameter if
    /// it depends (does not depend) on the value of the
    /// domain dynamic parameters (domain variables).
    DynamicP,
    //
    // Variable
    /// An AD object is a variable if
    /// it depends on the value of the domain variables.
    Variable,
    //
    // Empty
    /// This is used for the case where there is no information in this value
    Empty,
}
impl ADType {
    //
    /// is a constant parameter
    pub fn is_constant(&self) -> bool
    {   *self == ADType::ConstantP }
    //
    /// is a dynamic parameter
    pub fn is_dynamic(&self) -> bool
    {   *self == ADType::DynamicP }
    //
    /// is a parameter
    pub fn is_parameter(&self) -> bool
    {   *self == ADType::ConstantP ||  *self == ADType::DynamicP }
    //
    /// is a variable
    pub fn is_variable(&self) -> bool
    {   *self == ADType::Variable }
}
///
#[test]
fn test_ad_type() {
    assert!( ADType::ConstantP < ADType::DynamicP );
    assert!( ADType::DynamicP  < ADType::Variable );
    assert!( ADType::Variable  < ADType::Empty );
}
// ---------------------------------------------------------------------------
/// Documentation for the rustad generic type parameter V.
///
/// The generic parameter *V* , in ``AD`` < *V* > and other generic types ,
/// is the type used for calculating values.
/// It does not have dependency information that represents
/// how each value is related to the domain variables (independent variables).
pub fn doc_generic_v() {}
//
// AD
/// AD acts like V but in addition can record a function evaluation.
///
/// * V : see [doc_generic_v]
///
#[derive(Clone, Debug)]
pub struct AD<V> {
    //
    // tape_id
    ///
    /// This is the tape_id that the value of index below corresponds to.
    /// 1.  The tape_id zero never gets recorded.
    ///     The value of index and ad_type do not matter for this case.
    /// 2.  This object is a constant parameter if its tape_id is different
    ///     from the tape_id for this thread's tape.
    ///     The value of index and ad_type do not matter for this case.
    pub(crate) tape_id   : usize,
    //
    // index
    /// If this AD object's tape_id is the same as this thread's tape_id,
    /// *index* is the index in this thread's tape for this AD object.
    pub(crate) index : usize,
    //
    // ad_type
    /// If this AD object's tape_id is the same as this thread's tape_id,
    /// *ad_type* is Variable or DynamicP and is the type of this AD object.
    pub(crate) ad_type : ADType,
    //
    // value
    /// is the value of this AD object.
    pub(crate) value : V,
}
//
// new
impl<V> AD<V> {
    //
    /// Create an arbitrary new AD object.
    ///
    /// * new_tape_id : is the [AD::tape_id] for the new object.
    ///
    /// * new_index : is the [AD::index] for the new object.
    ///
    /// *ad_type : is the [AD::ad_type] for the new object.
    ///
    /// * new_value : is the [AD::value] for the new object.
    pub(crate) fn new(
        new_tape_id   : usize,
        new_index     : usize,
        new_ad_type   : ADType,
        new_value     : V,
    )-> Self {
        Self {
            tape_id   : new_tape_id,
            index     : new_index,
            ad_type   : new_ad_type,
            value     : new_value,
        }
    }
}
//
// to_value
impl<V> AD<V> {
    //
    /// Convert an AD object to a value
    /// (its the variable information is lost).
    ///
    /// **See Also** : example in [ad_from_value]
    ///
    /// # Example using NumVec
    /// ```
    /// use rustad::AD;
    /// use rustad::ad_from_value;
    /// use rustad::NumVec;
    /// type V  = rustad::AzFloat<f32>;
    /// let v   = vec![ V::from(2.0), V::from(3.0) ];
    /// let nv  = NumVec::new(v);
    /// let av  = ad_from_value(nv);
    /// let nv  = av.to_value();
    /// assert_eq!( nv.get(0), V::from(2.0) );
    /// assert_eq!( nv.get(1), V::from(3.0) );
    /// ```
    pub fn to_value(self) -> V {
        self.value
    }
}
// ---------------------------------------------------------------------------
// Display
//
/// Display only shows the value and ignores the variable information.
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// type V  = rustad::AzFloat<f64>;
/// let x   = V::from(5.0);
/// let ax  = ad_from_value(x);
/// let s   = format!( "{ax}" );
/// assert_eq!(s, "5");
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// use rustad::NumVec;
/// type S     = AzFloat<f32>;
/// type V     = NumVec<S>;
/// let x      = vec![ S::from(5.0), S::from(6.0) ];
/// let x_nv   = NumVec::new(x);
/// let ax     = ad_from_value(x_nv);
/// let s      = format!( "{ax}" );
/// assert_eq!(s, "[ 5, 6, ]");
/// ```
impl<V : std::fmt::Display> std::fmt::Display for AD<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// ---------------------------------------------------------------------------
// ad_from_value
/// Convert a value to an AD object with no function information;
/// i.e., a constant parameter.
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// type V  = rustad::AzFloat<f64>;
/// let x   = V::from(3.0);
/// let ax  = ad_from_value(x);
/// assert_eq!( ax.to_value(), V::from(3.0) );
/// ```
pub fn ad_from_value<V>(value : V) -> AD<V> {
    let tape_id   = 0;
    let index     = 0;
    let ad_type   = ADType::ConstantP;
    AD::new(tape_id, index, ad_type, value)
}
// ---------------------------------------------------------------------------
// ad_from_vector
/// Convert a vector to an vector of AD objects with no function information;
/// i.e., a vector of constant parameters.
///
/// **See Also** : example in [ad_from_value]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_vector;
/// type V   = rustad::AzFloat<f32>;
/// let x    = vec![ V::from(3.0), V::from(4.0) ];
/// let ax   = ad_from_vector(x);
/// assert_eq!( ax[0].clone().to_value(), V::from(3.0) );
/// assert_eq!( ax[1].clone().to_value(), V::from(4.0) );
/// ```
pub fn ad_from_vector<V> ( vec : Vec<V> ) -> Vec< AD<V> > {
    assert_ne!( vec.len() , 0 );
    let avec      = vec.into_iter().map( |value| {
        let tape_id   = 0;
        let index     = 0;
        let ad_type   = ADType::ConstantP;
        AD::new(tape_id, index, ad_type, value)
    } ).collect();
    avec
}
// ---------------------------------------------------------------------------
// ad_to_vector
/// Convert a vector of AD object to a vector of values
/// (any variable information is lost).
///
/// **See Also** : example in [ad_from_vector], [AD::to_value]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// use rustad::ad_to_vector;
/// type V    = rustad::AzFloat<f64>;
/// let ax    = vec![ ad_from_value(V::from(3)), ad_from_value(V::from(4)) ];
/// let y     = ad_to_vector(ax);
/// assert_eq!( y , vec![ V::from(3), V::from(4) ] );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::NumVec;
/// use rustad::ad_from_vector;
/// use rustad::ad_to_vector;
/// type S     = AzFloat<f32>;
/// type V     = NumVec<S>;
/// let v_0    = vec![ S::from(2.0), S::from(3.0) ];
/// let nv_0   = NumVec::new(v_0);
/// let v_1    = vec![ S::from(4.0), S::from(5.0) ];
/// let nv_1   = NumVec::new(v_1);
/// let av     = ad_from_vector( vec![nv_0, nv_1] );
/// let v      = ad_to_vector(av);
/// assert_eq!( v[0].get(0), S::from(2.0) );
/// assert_eq!( v[0].get(1), S::from(3.0) );
/// assert_eq!( v[1].get(0), S::from(4.0) );
/// assert_eq!( v[1].get(1), S::from(5.0) );
/// ```
pub fn ad_to_vector<V> ( avec : Vec< AD<V> > ) -> Vec<V> {
    assert_ne!( avec.len() , 0 );
    let vec  = avec.into_iter().map( |ad| ad.value).collect();
    vec
}
// -------------------------------------------------------------------------
/// Convert an f32 value to an AD object with no function information;
/// i.e., constant parameter.
///
/// See Also :
/// example in [ad_from_value], [ad_from_vector]
///
/// Syntax :
/// ```text
/// az  = AD::<V>::from( f32_value )
/// ```
///
/// * V : see [doc_generic_v]
///
/// * f32_value : is an f32 value
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::NumVec;
/// use rustad::AzFloat;
/// type S = rustad::AzFloat<f32>;
/// type V = NumVec<S>;
/// let ax = AD::<V>::from(3.0 as f32);
/// let x  = ax.to_value();
/// assert_eq!( x.get(0).to_inner(), 3.0 as f32);
/// ```
impl<V> From<f32> for AD<V>
where
    V : From<f32>,
{
    fn from( f32_value : f32 ) -> AD<V> {
        let tape_id    = 0;
        let index      = 0;
        let ad_type    = crate::ad::ADType::ConstantP;
        let value      = V::from(f32_value);
        AD::new(tape_id, index, ad_type, value)
    }
}
// -------------------------------------------------------------------------
/// Convert an f64 value to an AD object with no function information;
/// i.e., constant parameter.
///
/// See Also :
/// example in [ad_from_value], [ad_from_vector]
///
/// Syntax :
/// ```text
/// az = AD<V>::from( f64_value )
/// ```
///
/// * V : see [doc_generic_v] . In addition, this type must support
/// `V::from<f64>` .
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::NumVec;
/// type S  = AzFloat<f64>;
/// type V  = NumVec<S>;
/// let ax  = AD::<V>::from(3.0 as f64);
/// let x   = ax.to_value();
/// assert_eq!( x.get(0), S::from(3.0) );
/// ```
impl<V> From<f64> for AD<V>
where
    V : From<f64>,
{
    fn from( f64_value : f64 ) -> AD<V> {
        let tape_id    = 0;
        let index      = 0;
        let ad_type    = crate::ad::ADType::ConstantP;
        let value      = V::from(f64_value);
        AD::new(tape_id, index, ad_type, value)
    }
}
// -------------------------------------------------------------------------
/// Convert V to an `AD<V>` object with no function information;
/// i.e., constant parameter.
///
/// See Also :
/// example in [ad_from_value], [ad_from_vector]
///
/// Syntax :
/// ```text
/// av = AD<V>::from( v )
/// ```
///
/// * v : is an [doc_generic_v] object.
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// type V  = AzFloat<f32>;
/// let  v  = V::from(3.0 as f32);
/// let ax  = AD::<V>::from(v);
/// let x   = ax.to_value();
/// assert_eq!( x, V::from(3.0) );
/// ```
impl<B> From< AzFloat<B> > for AD< AzFloat<B> >
{
    fn from( value : AzFloat<B> ) -> AD< AzFloat<B> > {
        let tape_id    = 0;
        let index      = 0;
        let ad_type    = crate::ad::ADType::ConstantP;
        AD::new(tape_id, index, ad_type, value)
    }
}
