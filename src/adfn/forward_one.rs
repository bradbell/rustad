// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_one method (directional derivatives).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::AD;
use crate::ADfn;
use crate::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
#[cfg(doc)]
use crate::adfn::forward_zero::doc_forward_zero;
//
// -----------------------------------------------------------------------
// forward_one
/// First order forward mode evaluation; i.e., directional derivatives.
///
/// * Syntax :
/// ```text
///     range_one = f.forward_one_value(&var_both, domain_one, trace)
///     range_one = f.forward_one_ad(   &var_both, domain_one, trace)
/// ```
///
/// * Prototype :
/// see [ADfn::forward_one_value] and [ADfn::forward_one_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// ## f
/// is an [ADfn] object.
///
/// ## var_both
/// must be the *var_both* computed a the previous call to forward_zero.
///
/// ## domain_one
/// specifies the domain space direction along which the directional
/// derivative is evaluated.
///
/// ## trace
/// if true, a trace of the calculations is printed on stdout.
///
/// ## range_one
/// The return value is the directional derivative
/// ```text
///     range_one = f'(domain_zero) * domain_one
/// ```
/// Here `f'` is the derivative of the function and
/// [domain_zero](doc_forward_zero#domain_zero) is its value in the call to
/// forward_zero that created the *var_both* .
///
/// # Example
/// Computing one partial derivative using forward_one :
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::ad_from_value;
///
/// // V
/// type V = f64;
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
/// let mut v0 : Vec<V> = Vec::new();
/// let y0              = f.forward_zero_value(&mut v0, x0, trace);
/// let x1     : Vec<V> = vec![ 1.0, 0.0, 0.0 ];
/// let y1              = f.forward_one_value(&v0, x1,  trace);
/// //
/// assert_eq!( y1[0] , 5.0 * 6.0 );
/// ```
///
pub fn doc_forward_one() { }
//
/// Create the first order forward mode member functions.
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
/// See [doc_forward_one]
macro_rules! forward_one {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of first order forward mode; ",
            "see [doc_forward_one]",
        )]
        pub fn [< forward_one_ $suffix >] (
            &self,
            var_both    : &Vec<$E>     ,
            domain_one  : Vec<$E>      ,
            trace       : bool         ,
        ) -> Vec<$E>
        {
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            assert_eq!(
                domain_one.len(), self.var.n_dom,
                "f.forward_one: domain vector length does not match f"
            );
            assert_eq!(
                var_both.len(), n_var,
                "f.forward_one: var_both does not have the correct length"
            );
            //
            // dyp_both
            let dyp_both : Vec<$E> = Vec::new();
            //
            // range_one
            let range_one = self.[< forward_der_ $suffix >] (
                &dyp_both, var_both, domain_one, trace
            );
            range_one
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // forward_one
    forward_one!( value, V, V );
    forward_one!( ad,    V, AD::<V> );
}
