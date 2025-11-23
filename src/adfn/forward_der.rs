// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_der method (directional derivatives).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::AD;
use crate::ADfn;
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::adfn::eval_from::eval_from_f32;
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
// forward_der
/// First order forward mode evaluation with dynamic parameters.
///
/// * Syntax :
/// ```text
///     range_der = f.forward_der_value(&dyp_both, &var_both, dom_der, trace)
///     range_der = f.forward_der_ad(&dyp_both, &var_both, dom_der, trace)
/// ```
///
/// * Prototype :
/// see [ADfn::forward_der_value] and [ADfn::forward_der_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * dyp_both :
/// is both the dynamic parameter sub-vectors in the following order:
/// the domain dynamic parameters followed by the dependent dynamic parameters.
/// This is normally computed by
/// [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) .
/// In the special case where there are no dynamic parameters,
/// *dyp_both* can be set to the empty vector
/// ( it is not necessary to call `forward_dyp` ).
///
/// * var_both :
/// is both the variable sub-vectors in the following order:
/// the domain variables followed by the dependent variables.
/// This is normally computed by
/// [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// * domain_der :
/// specifies the domain space direction along which the directional
/// derivative is evaluated. This is a direction in variable space.
///
/// * trace :
/// if true, a trace of the calculations is printed on stdout.
///
/// * range_der
/// The return value is the directional derivative; i.e,
/// ```text
///     range_der = f_var (dyp_dom, var_dom )  * domain_der
/// ```
/// Here `f_var` is the derivative of f with respect to the variables,
/// `dyp_dom` is its value in the call to
/// [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) , and
/// `var_dom` is its value in the call to
/// [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// # Example
/// Computing one partial derivative using forward_der :
/// ```
/// use rustad::start_recording_dyp_var;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::ad_from_value;
///
/// // V
/// type V = rustad::AzFloat<f64>;
/// //
/// // f
/// // f(p, x) = p[0] * p[1] * x[0] * x[1] * x[2]
/// let p    : Vec<V>   = vec![ V::from(1.0), V::from(1.0) ];
/// let x    : Vec<V>   = vec![ V::from(1.0), V::from(1.0), V::from(1.0) ];
/// let (ap, ax )       = start_recording_dyp_var(p, x);
/// let aterm1          = &ap[0] * &ap[1];
/// let aterm2          = &( &ax[0] * &ax[1] ) * &ax[2];
/// let aprod           = &aterm1 * &aterm2;
/// let ay              = vec![ aprod ];
/// let f               = stop_recording(ay);
/// //
/// // dy = partial f(p, x) w.r.t. x[0]
/// let trace           = false;
/// let p      : Vec<V> = vec![ V::from(2.0), V::from(3.0) ];
/// let x      : Vec<V> = vec![ V::from(4.0), V::from(5.0), V::from(6.0) ];
/// let dyp             = f.forward_dyp_value(p, trace);
/// let (y, var)        = f.forward_var_value(&dyp, x, trace);
/// let dx     : Vec<V> = vec![ V::from(1.0), V::from(0.0), V::from(0.0) ];
/// let dy              = f.forward_der_value(&dyp, &var, dx,  trace);
/// //
/// // check
/// // derivative w.r.t x[0] is p[0] * p[1] * x[1] * x[2] * x[3]
/// assert_eq!( dy[0] , V::from( 2.0 * 3.0 * 5.0 * 6.0 ) );
/// ```
///
pub fn doc_forward_der() { }
//
/// Create the first order forward mode member functions.
///
/// * suffix : is either `value` or `ad` ;
/// * V      : see [doc_generic_v]
/// * E      : see [doc_generic_e] .
///
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_forward_der]
macro_rules! forward_der {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of first order forward mode; ",
            "see [doc_forward_der]",
        )]
        pub fn [< forward_der_ $suffix >] (
            &self,
            dyp_both    : &Vec<$E>     ,
            var_both    : &Vec<$E>     ,
            dom_der     : Vec<$E>      ,
            trace       : bool         ,
        ) -> Vec<$E>
        {
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            assert_eq!( dyp_both.len(), n_dyp,
                "f.forward_der: dyp_both vector length does not match f"
            );
            assert_eq!(
                var_both.len(), n_var,
                "f.forward_der: var_both vector length does not match f"
            );
            //
            assert_eq!(
                dom_der.len(), self.var.n_dom,
                "f.forward_der: dom_der vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_e        : $E = eval_from_f32!($suffix, $V, f32::NAN);
            //
            // var_der
            let nan_e         : $E = eval_from_f32!($suffix, $V, f32::NAN);
            let mut var_der        = dom_der;
            var_der.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_der: n_var = {}", n_var);
                println!( "index, flag" );
                for j in 0 .. self.var.flag_all.len() {
                    println!( "{}, {:?}", j, self.var.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "index, dyp_both" );
                for j in 0 .. n_dyp {
                    println!( "{}, {}", j, dyp_both[j] );
                }
                println!( "var_index, var_dom, dom_der" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}, {}", j, var_both[j], var_der[j] );
                }
                println!( "var_index, var_both, var_der, op_name, arg" );
            }
            //
            // var_der
            for op_index in 0 .. self.var.id_seq.len() {
                let op_id    = self.var.id_seq[op_index] as usize;
                let start    = self.var.arg_seq[op_index] as usize;
                let end      = self.var.arg_seq[op_index + 1] as usize;
                let arg      = &self.var.arg_all[start .. end];
                let arg_type = &self.var.arg_type_all[start .. end];
                let res      = self.var.n_dom + op_index;
                let forward_der = op_info_vec[op_id].[< forward_der_ $suffix >];
                forward_der(
                    &dyp_both,
                    &var_both,
                    &mut var_der,
                    &self.cop,
                    &self.var.flag_all,
                    &arg,
                    &arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {}, {:?}",
                        res, var_both[res], var_der[res], name, arg
                    );
                }
            }
            if trace {
                println!( "range_index, var_index, con_index" );
                for i in 0 .. self.range_ad_type.len() {
                    let index = self.range_index[i] as usize;
                    if self.range_ad_type[i].is_variable() {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_der" );
            }
            let mut range_one : Vec<$E> = Vec::new();
            for i in 0 .. self.range_ad_type.len() {
                let index = self.range_index[i] as usize;
                if self.range_ad_type[i].is_variable() {
                    range_one.push( var_der[index].clone() );
                } else {
                    range_one.push( zero_e.clone() );
                }
            }
            range_one
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // forward_der
    forward_der!( value, V, V );
    forward_der!( ad,    V, AD::<V> );
}
