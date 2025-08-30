// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD function objects
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
#[cfg(doc)]
use crate::numvec::ad::doc_generic_v;
#[cfg(doc)]
use crate::numvec::ad::AD;
//
// Tindex
use crate::numvec::tape::Tindex;
//
// ---------------------------------------------------------------------------
/// Documentation for the rustad generic type parameter E.
///
/// This is the type used to evaluate ADFn member functions with names
/// that begin with
/// ``forward`` , ``reverse`` , ``ad_forward`` , ``ad_reverse`` .
///
/// *   If a member function name begins with `forward` or ``reverse`` ,
///     *E* is the same as *V*; see [doc_generic_v] and [AD].
///
/// *   If a member function name begins with `ad_forward` or ``ad_reverse`` ,
///     *E* is the same as ``AD`` < *V* >; see [doc_generic_v] and [AD].
///
/// Note that *V* evaluations are used to compute values and
/// `AD<V>` evaluations are used to build new AD function objects.
///
pub fn doc_generic_e() {}
// ---------------------------------------------------------------------------
// ADFn
//
/// An ADFn can an evaluate the function and its derivatives
/// corresponding to an ``AD`` < *V* > operation sequence.
///
/// * V : [doc_generic_v]
///
/// * Operation sequence :
/// An operation sequence is a single assignment representation of
/// a function; i.e., each variable is only assigned once.
///
/// * TODO : Change member variables to pub(crate) (once they get used).
pub struct ADFn<V> {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain     : usize,
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub n_var               : usize,
    //
    // id_all
    /// This maps each operator's index in the operation sequence
    /// to its operator id; see operator::id.
    pub id_all              : Vec<u8>,
    //
    // flag_all
    /// This contains all the boolean flags that are part of
    /// the operator definitions.
    pub flag_all            : Vec<bool>,
    //
    // op2arg
    /// This maps each operator's index in the operation sequence to
    /// the index of its first argument in arg_all.
    pub op2arg              : Vec<Tindex>,
    //
    // arg_all
    /// This contains all the arguments for the opereators in the
    /// operatioon sequence.
    pub arg_all             : Vec<Tindex>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub con_all             : Vec<V>,
    //
    // range_is_var
    /// The length of this vector is the dimension of the range space.
    /// If range_is_var\[i\] is true (false), the i-th range space component
    /// is a variable (constant).
    pub(crate) range_is_var : Vec<bool>,
    //
    // range2tape_index
    /// The length of this vector is also the dimension of the range space.
    /// If range_is_var\[i\] is true (false), range2tape_indx\[i\] is the
    /// variable (constant) index for the i-th component of the range space.
    pub range2tape_index    : Vec<Tindex>,
}
//
// ---------------------------------------------------------------------------
impl<V> ADFn<V> {
    //
    // new
    /// This creates an ADFn object with an empty operation sequence.
    ///
    /// To be more specific,
    /// the corresponding  domain and range vectors have length zero.
    ///
    /// # Example
    /// ```
    /// use rustad::numvec::adfn::ADFn;
    /// let f : ADFn<f32> = ADFn::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain         : 0,
                             n_var            : 0,
                             id_all           : Vec::new() ,
            flag_all         : Vec::new() ,
            op2arg           : Vec::new() ,
            arg_all          : Vec::new() ,
            con_all          : Vec::new() ,
            range_is_var     : Vec::new() ,
            range2tape_index : Vec::new() ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> usize { self.n_domain }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> usize { self.range_is_var.len() }
}
