// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines AD function objects
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// sub-modules
//
pub mod forward_dyp;
pub mod forward_var;
pub mod forward_der;
pub mod reverse_der;
pub mod sub_sparsity;
pub mod for_sparsity;
pub mod for_sparse_jac;
pub mod rust_src;
pub mod optimize;
//
// ---------------------------------------------------------------------------
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    AD,
};
//
// ADType
use crate::ad::ADType;
//
// IndexT
use crate::IndexT;
//
// OpSequence
use crate::tape::OpSequence;
//
// Sparsity
pub type SparsityPattern = Vec<[usize; 2]>;
//
// ---------------------------------------------------------------------------
/// Documentation for the rustad generic type parameter E.
///
/// This is the type used to evaluate ADfn member functions with names
/// that begin with ``forward`` or ``reverse`` .
///
/// *   If a member function name ends with `_value` ,
///     *E* is the same as *V*; see [doc_generic_v] and [AD].
///
/// *   If a member function name ends with `_ad` ,
///     *E* is the same as ``AD`` < *V* >; see [doc_generic_v] and [AD].
///
/// Note that *V* evaluations are used to compute values and
/// `AD<V>` evaluations are used to build new [ADfn] objects.
///
pub fn doc_generic_e() {}
// ---------------------------------------------------------------------------
// ADfn
//
/// An ADfn can an evaluate the function and its derivatives
/// corresponding to an ``AD`` < *V* > operation sequence.
///
/// * V : [doc_generic_v]
///
pub struct ADfn<V> {
    //
    // dyp
    // The dynamic parameeter operation sequence
    pub(crate) dyp : OpSequence,
    //
    // var
    // The variable operation sequence
    pub(crate) var : OpSequence,
    //
    // rng_ad_type
    /// The length of this vector is the dimension of the range space.
    /// If rng_ad_type\[i\] is Variable (DynamicP) {ConstantP},
    /// the i-th range space component is a
    /// variable (dynamic parameter) {constant parameter}.
    pub(crate) rng_ad_type   : Vec<ADType>,
    //
    // rng_index
    /// The length of this vector is also the dimension of the range space.
    /// If rng_ad_type\[i\] is Variable (DynamicP) {ConstantP},
    /// rng_index\[i]\ is a variable index
    /// (dynamic parameter index) {constant parameter index} .
    pub(crate) rng_index           : Vec<IndexT>,
    //
    // cop
    /// is the vector of constant parameters used by both operation sequences.
    pub(crate) cop : Vec<V>,
}
//
// ---------------------------------------------------------------------------
// ADfn<V>::default
impl<V> Default for ADfn<V> {
    /// This creates an ADfn object with an empty operation sequence.
    ///
    /// To be more specific,
    /// the corresponding  domain and range vectors have length zero.
    ///
    /// # Example
    /// ```
    /// use rustad::adfn::ADfn;
    /// let f : ADfn<f32> = ADfn::default();
    /// assert_eq!( f.dyp_dom_len() + f.dyp_dep_len(), 0 );
    /// assert_eq!( f.var_dom_len() + f.var_dep_len(), 0 );
    /// assert_eq!( f.rng_len(), 0 );
    /// assert_eq!( f.cop_len(), 0 );
    /// ```
    fn default() -> Self {
        Self {
            dyp              : OpSequence::new(),
            var              : OpSequence::new(),
            rng_ad_type      : Vec::new() ,
            rng_index        : Vec::new() ,
            cop              : Vec::new() ,
        }
    }
}
impl<V> ADfn<V> {
    //
    // dyp_dom_len
    /// number of domain dynamic parameters
    pub fn dyp_dom_len(&self) -> usize { self.dyp.n_dom }
    //
    // dyp_dep_len
    /// number of dependent dynamic parameters
    pub fn dyp_dep_len(&self) -> usize { self.dyp.n_dep }
    //
    // dyp_len
    /// number of dynamic parameters
    pub fn dyp_len(&self) -> usize { self.dyp.n_dom  + self.dyp.n_dep }
    //
    // var_dom_len
    /// number of domain variables
    pub fn var_dom_len(&self) -> usize { self.var.n_dom }
    //
    // var_dep_len
    /// number of dependent variables
    pub fn var_dep_len(&self) -> usize { self.var.n_dep }
    //
    // var_len
    /// number of variables
    pub fn var_len(&self) -> usize { self.var.n_dom  + self.var.n_dep }
    //
    // rng_len
    /// dimension of range space
    pub fn rng_len(&self) -> usize {
        debug_assert!( self.rng_index.len() == self.rng_ad_type.len() );
        self.rng_ad_type.len()
    }
    //
    // cop_len
    /// number of constant parameters in this function
    pub fn cop_len(&self) -> usize { self.cop.len() }
    //
    // swap
    /// exchange the contents of this ADfn with another ADfn.
    pub fn swap(&mut self, other : &mut ADfn<V>) {
        std::mem::swap( &mut self.var,           &mut other.var );
        std::mem::swap( &mut self.cop,           &mut other.cop );
        std::mem::swap( &mut self.rng_ad_type, &mut other.rng_ad_type );
        std::mem::swap( &mut self.rng_index,   &mut other.rng_index );
    }
}
