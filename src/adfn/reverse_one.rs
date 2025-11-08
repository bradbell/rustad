// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] reverse_one method (partial derivatives).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
};
use crate::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
#[cfg(doc)]
use crate::adfn::forward_zero::doc_forward_zero;
// -----------------------------------------------------------------------
// reverse_one
/// First order reverse mode evaluation with no dynamic parameters.
///
/// * Syntax :
/// ```text
///     dom_der = f.reverse_one_value(&var_both, range_der, trace)
///     dom_der = f.reverse_one_ad(&var_both, range_der, trace)
/// ```
///
/// * Prototype :
/// see [ADfn::reverse_one_value] and [ADfn::reverse_one_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * Other Arguments :
/// This is a wrapper for
/// [reverse_der](crate::adfn::reverse_der::doc_reverse_der)
/// that fills in an empty vector for dyp_both
///
/// # Example
/// Computing all the partial derivatives using reverse_one :
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::ad_from_value;
///
/// // V
/// type V = f32;
/// //
/// // f
/// // f(x) = x[0] * x[1] * x[2]
/// let x        : Vec<V> = vec![ 1.0, 1.0, 1.0 ];
/// let ax                  = start_recording(x);
/// let mut aprod           = ad_from_value( V::from(1.0) );
/// for j in 0 .. ax.len() {
///     aprod *= &ax[j];
/// }
/// let ay = vec![ aprod ];
/// let f  = stop_recording(ay);
/// //
/// // y
/// // y[0] = f(x)
/// let trace           = false;
/// let x0     : Vec<V> = vec![ 4.0, 5.0, 6.0 ];
/// let (_, v0)         = f.forward_zero_value(x0, trace);
/// let y1     : Vec<V> = vec![ 1.0 ];
/// let x1              = f.reverse_one_value(&v0, y1, trace);
/// //
/// assert_eq!( x1[0] , 5.0 * 6.0 );
/// assert_eq!( x1[1] , 4.0 * 6.0 );
/// assert_eq!( x1[2] , 4.0 * 5.0 );
/// ```
///
pub fn doc_reverse_one() { }
//
/// Create the first order reverse mode member functions.
///
/// * suffix :
/// is either `value` or `ad` ;
///
/// * V : see [doc_generic_v]
///
/// * E : see [doc_generic_e] .
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_reverse_one]
macro_rules! reverse_one {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of first order reverse mode; ",
            "see [doc_reverse_one]",
        )]
        pub fn [< reverse_one_ $suffix >] (
            &self,
            var_both    : &Vec<$E>  ,
            range_der   : Vec<$E>   ,
            trace       : bool      ,
        ) -> Vec<$E>
        {
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            assert_eq!(
                range_der.len(), self.range_ad_type.len(),
                "f.reverse_one: range vector length does not match f"
            );
            assert_eq!(
                 var_both.len(), n_var,
                "f.reverse_one:  var_both does not have the proper length"
            );
            //
            // dyp_both
            let dyp_both : Vec<$E> = Vec::new();
            //
            // dom_der
            let dom_der = self.[< reverse_der_ $suffix >](
                &dyp_both, &var_both, range_der, trace
            );
            dom_der
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // reverse_one
    reverse_one!( value, V, V );
    reverse_one!( ad,    V, AD::<V> );
}
