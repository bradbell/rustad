// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] reverse_der method (partial derivatives).
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
// -----------------------------------------------------------------------
// reverse_der
/// First order reverse mode evaluation with dynamic parameters.
///
/// * Syntax :
/// ```text
///     dom_der = f.reverse_der_value(&dyp_both, &var_both, range_der, trace)
///     dom_der = f.reverse_der_ad(&dyp_both, &var_both, range_der, trace)
/// ```
///
/// * Prototype :
/// see [ADfn::reverse_der_value] and [ADfn::reverse_der_ad]
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
/// * range_der :
/// specifies the range space weights that define the scalar function
/// that this call will evaluate the gradient for.
///
/// * trace :
/// if true, a trace of the operations is printed on stdout.
///
/// The return value *dom_der* is the gradient of *range_der* times
/// the derivative of f with respect to the variables; i.e.,
/// ```text
///     dom_der = range_der * f_var (dyp_dom, var_dom)
/// ```
/// Here `f_var` is the derivative of f with respect to the variables,
/// `dyp_dom` is its value in the call to
/// [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) , and
/// `var_dom` is its value in the call to
/// [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// # Example
/// Computing the gradient using reverse_der :
/// ```
/// use rustad::start_recording_dyp_var;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
///
/// // V
/// type V = rustad::AzFloat<f32>;
/// // f
/// // f(x) = p[0] * p[1] * x[0] * x[1] * x[2]
/// let p          = vec![ V::from(1.0), V::from(1.0) ];
/// let x          = vec![ V::from(1.0), V::from(1.0), V::from(1.0) ];
/// let (ap, ax )  = start_recording_dyp_var(p, x);
/// let aterm1     = &ap[0] * &ap[1];
/// let aterm2     = &( &ax[0] * &ax[1] ) * &ax[2];
/// let aprod      = &aterm1 * &aterm2;
/// let ay         = vec![ aprod ];
/// let f          = stop_recording(ay);
/// //
/// // dx = derivative of f(p, x) with respect to x
/// let trace      = false;
/// let p          = vec![ V::from(2.0), V::from(3.0) ];
/// let x          = vec![ V::from(4.0), V::from(5.0), V::from(6.0) ];
/// let dyp        = f.forward_dyp_value(p.clone(), trace);
/// let (y, var)   = f.forward_var_value(&dyp, x.clone(), trace);
/// let dy         = vec![ V::from(1.0) ];
/// let dx         = f.reverse_der_value(&dyp, &var, dy,  trace);
/// //
/// assert_eq!( dx[0] , p[0] * p[1] * x[1] * x[2] );
/// assert_eq!( dx[1] , p[0] * p[1] * x[0] * x[2] );
/// assert_eq!( dx[2] , p[0] * p[1] * x[0] * x[1] );
/// ```
///
pub fn doc_reverse_der() { }
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
/// See [doc_reverse_der]
macro_rules! reverse_der {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of first order reverse mode; ",
            "see [doc_reverse_der]",
        )]
        pub fn [< reverse_der_ $suffix >] (
            &self,
            dyp_both    : &Vec<$E>  ,
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
                "f.reverse_der: range vector length does not match f"
            );
            assert_eq!(
                 var_both.len(), n_var,
                "f.reverse_der:  var_both does not have the proper length"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_e : $E = eval_from_f32!($suffix, $V, 0 as f32);
            //
            // var_der
            let mut var_der       = vec![ zero_e; n_var ];
            let mut mut_range_der = range_der;
            for i in (0 .. self.range_ad_type.len()).rev() {
                let y_i = mut_range_der.pop().unwrap();
                if self.range_ad_type[i].is_variable() {
                    let index = self.range_index[i] as usize;
                    var_der[index] = y_i;
                }
            }
            //
            if trace {
                println!( "Begin Trace: reverse_der: n_var = {}", n_var);
                println!( "index, flag" );
                for j in 0 .. self.var.flag_all.len() {
                    println!( "{}, {:?}", j, self.var.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "index, dyp_both" );
                for j in 0 .. dyp_both.len() {
                    println!( "{}, {}", j, dyp_both[j] );
                }
                println!( "var_index, range_der" );
                for i in 0 .. self.range_ad_type.len() {
                    if self.range_ad_type[i].is_variable() {
                        let index = self.range_index[i] as usize;
                        println!( "{}, {}", index,  var_der[index] );
                    }
                }
                println!( "var_index, var_both, var_der, op_name, arg" );
            }
            //
            // var_der
            for op_index in ( 0 .. self.var.id_seq.len() ).rev() {
                let op_id     = self.var.id_seq[op_index] as usize;
                let start     = self.var.arg_seq[op_index] as usize;
                let end       = self.var.arg_seq[op_index + 1] as usize;
                let arg       = &self.var.arg_all[start .. end];
                let arg_type  = &self.var.arg_type_all[start .. end];
                let res       = self.var.n_dom + op_index;
                let reverse_1 = op_info_vec[op_id].[< reverse_der_ $suffix >];
                reverse_1(
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
                println!( "End Trace: reverse_der" );
            }
            //
            // domain_der
            let nan_e  : $E    = eval_from_f32!($suffix, $V,  f32::NAN);
            let mut domain_der = var_der;
            domain_der.resize(self.var.n_dom, nan_e);
            domain_der.shrink_to_fit();
            domain_der
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // reverse_der
    reverse_der!( value, V, V );
    reverse_der!( ad,    V, AD::<V> );
}
