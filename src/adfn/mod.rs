// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD function objects
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// sub-modules
//
pub mod forward_zero;
pub mod forward_one;
pub mod reverse_one;
pub mod sub_sparsity;
pub mod for_sparsity;
pub mod rust_src;
//
pub(crate) mod eval_from;
// ---------------------------------------------------------------------------
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    AD,
};
//
// IndexT
use crate::IndexT;
//
// OpSequence
use crate::tape::OpSequence;
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
    // var
    // The variable operation sequence
    pub(crate) var : OpSequence,
    //
    // range_is_var
    /// The length of this vector is the dimension of the range space.
    /// If range_is_var\[i\] is true (false), the i-th range space component
    /// is a variable (constant).
    pub(crate) range_is_var : Vec<bool>,
    //
    // range2tape_index
    /// The length of this vector is also the dimension of the range space.
    /// If range_is_var\[i\] is true (false), range2tape_index\[i\] is the
    /// variable (constant) index for the i-th component of the range space.
    pub(crate) range2tape_index    : Vec<IndexT>,
    //
    // cop
    /// is the vector of constant parameters used by both operation sequences.
    pub(crate) cop : Vec<V>,
}
//
// ---------------------------------------------------------------------------
impl<V> ADfn<V> {
    //
    // new
    /// This creates an ADfn object with an empty operation sequence.
    ///
    /// To be more specific,
    /// the corresponding  domain and range vectors have length zero.
    ///
    /// # Example
    /// ```
    /// use rustad::adfn::ADfn;
    /// let f : ADfn<f32> = ADfn::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            var              : OpSequence::new(),
            range_is_var     : Vec::new() ,
            range2tape_index : Vec::new() ,
            cop             : Vec::new()  ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> usize { self.var.n_dom }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> usize { self.range_is_var.len() }
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
        std::mem::swap( &mut self.range_is_var,  &mut other.range_is_var );
        std::mem::swap(
            &mut self.range2tape_index, &mut other.range2tape_index
        );
    }
}
