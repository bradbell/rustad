// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_zero method (function values).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn
};
use crate::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode variable evaluation with no dynamic parameters.
///
/// * Syntax :
/// ```text
///     (range, var_both) = f.forward_zero_value(var_dom, trace)
///     (range, var_both) = f.forward_zero_ad(var_dom, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_zero_value] and [ADfn::forward_zero_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * Other Arguments :
/// This is a wrapper for
/// [forward_var](crate::adfn::forward_var::doc_forward_var)
/// that fills in an empty vector for dyp_both .
///
/// # Example
/// Computing function values using forward_zero :
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// //
/// // V
/// type V = rustad::AzFloat<f32>;
/// //
/// // f
/// // f(x) = x[0] + ... + x[nx-1]
/// let x                 = vec![ V::from(1.0), V::from(1.0), V::from(1.0) ];
/// let ax                = start_recording(x);
/// let mut asum          = ad_from_value( V::from(0.0) );
/// for j in 0 .. ax.len() {
///     asum += &ax[j];
/// }
/// let ay = vec![ asum ];
/// let f  = stop_recording(ay);
/// //
/// // y
/// // y[0] = f(x)
/// let trace           = false;
/// let x      : Vec<V> = vec![ V::from(1.0), V::from(2.0), V::from(3.0) ];
/// let (y, _v)  = f.forward_zero_value(x, trace);
/// //
/// assert_eq!( y[0] , V::from(1 + 2 + 3) );
/// ```
///
pub fn doc_forward_zero() { }
//
/// Create the no dynamic parameter zero order forward mode member functions.
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
/// See [doc_forward_zero]
macro_rules! forward_zero {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of zero order forward mode; ",
            "see [doc_forward_zero]",
        )]
        pub fn [< forward_zero_ $suffix >] (
            &self,
            var_dom : Vec<$E>      ,
            trace   : bool         ,
        ) -> ( Vec<$E>, Vec<$E> )
        {   assert_eq!(
                var_dom.len(), self.var.n_dom,
                "f.forward_zero: var_dom length does not match f"
            );
            //
            // dyp_both
            let dyp_both : Vec<$E> = Vec::new();
            //
            // range, var_both
            let (range, var_both) =
                self. [< forward_var_ $suffix >]  (
                    &dyp_both, var_dom, trace
            );
            (range, var_both)
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // forward_zero
    forward_zero!( value, V, V );
    forward_zero!( ad,    V, AD::<V> );
}
