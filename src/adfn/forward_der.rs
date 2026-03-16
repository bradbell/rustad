// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_der method (directional derivatives).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
    FConst,
};
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
#[cfg(doc)]
//
// -----------------------------------------------------------------------
// forward_der
/// First order forward mode evaluation with dynamic parameters.
///
/// * Syntax :
///   ```text
///     range_der = f.forward_der_value(dyp_all, &var_all, dom_der, arg_vec)
///     range_der = f.forward_der_ad(dyp_all, &var_all, dom_der, arg_vec)
///   ```
///
/// * Prototype :
///   see [ADfn::forward_der_value] and [ADfn::forward_der_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * dyp_all  :
///   If there are no dynamic parameters in f, this should be None
///   or the empty vector.
///   Otherwise it is the dynamic parameter sub-vectors in the following order:
///   the domain dynamic parameters followed by the dependent dynamic parameters.
///   This is normally computed by
///   [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) .
///
/// * var_all  :
///   is both the variable sub-vectors in the following order:
///   the domain variables followed by the dependent variables.
///   This is normally computed by
///   [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// * domain_der :
///   specifies the domain space direction along which the directional
///   derivative is evaluated. This is a direction in variable space.
///
/// * arg_vec :
///   is an [arg_vec](crate::doc_arg_vec) with the following possible keys:
///
///   * trace
///     The corresponding value must be true of false (default is false).
///     If it is true, a trace of forward_der is printed on stdout.
///
/// * range_der
///   The return value is the directional derivative; i.e,
///   ```text
///     range_der = f_var (dyp_dom, var_dom )  * domain_der
///   ```
///   Here `f_var` is the derivative of f with respect to the variables,
///   `dyp_dom` is its value in the call to
///   [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) , and
///   `var_dom` is its value in the call to
///   [forward_var](crate::adfn::forward_var::doc_forward_var) .
///
/// # Example
/// Computing one partial derivative using forward_der :
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// use rustad::AD;
///
/// // V
/// type V = rustad::AzFloat<f64>;
/// //
/// // f
/// // f(p, x) = p[0] * p[1] * x[0] * x[1] * x[2]
/// let p    : Vec<V>   = vec![ V::from(1.0), V::from(1.0) ];
/// let x    : Vec<V>   = vec![ V::from(1.0), V::from(1.0), V::from(1.0) ];
/// let (ap, ax )       = start_recording(Some(p), x);
/// let aterm1          = &ap[0] * &ap[1];
/// let aterm2          = &( &ax[0] * &ax[1] ) * &ax[2];
/// let aprod           = &aterm1 * &aterm2;
/// let ay              = vec![ aprod ];
/// let f               = stop_recording(ay);
/// //
/// // dy = partial f(p, x) w.r.t. x[0]
/// let trace           = false;
/// let arg_vec : Vec<[&str; 2]> = Vec::new();
/// let p      : Vec<V> = vec![ V::from(2.0), V::from(3.0) ];
/// let x      : Vec<V> = vec![ V::from(4.0), V::from(5.0), V::from(6.0) ];
/// let dyp             = f.forward_dyp_value(p, &arg_vec);
/// let (y, var)        = f.forward_var_value(Some(&dyp), x, &arg_vec);
/// let dx     : Vec<V> = vec![ V::from(1.0), V::from(0.0), V::from(0.0) ];
/// let dy              = f.forward_der_value(Some(&dyp), &var, dx, &arg_vec);
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
/// * E      : see [doc_generic_e] .
///
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_forward_der]
macro_rules! forward_der {
    ( $suffix:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of first order forward mode; ",
            "see [doc_forward_der]",
        )]
        pub fn [< forward_der_ $suffix >] (
            &self,
            dyp_all     : Option< &Vec<$E> >  ,
            var_all     : &Vec<$E>            ,
            dom_der     : Vec<$E>             ,
            arg_vec     : &Vec<[&str; 2]>     ,
        ) -> Vec<$E>
        {
            // trace
            let mut trace = false;
            for arg in arg_vec {
                match arg[0] {
                    "trace" => {
                        match arg[1] {
                            "true"  => { trace = true; },
                            "false" => { trace = false; },
                            _ => { panic!(
                            "forward_der arg_vec: invalid value for trace"
                            ); }
                        }
                    },
                    _ => panic!("forward_der arg_vec: invalid key"),
                }
            }
            //
            // dyp_all
            let dyp_all  : &Vec<$E> = if dyp_all.is_none() {
                &Vec::new()
            } else {
                dyp_all.unwrap()
            };
            //
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            assert_eq!( dyp_all.len(), n_dyp,
                "f.forward_der: dyp_all vector length does not match f"
            );
            assert_eq!(
                var_all.len(), n_var,
                "f.forward_der: var_all vector length does not match f"
            );
            //
            assert_eq!(
                dom_der.len(), self.var.n_dom,
                "f.forward_der: dom_der vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_e             = $E::zero();
            //
            // var_der
            let nan_e              = $E::nan();
            let mut var_der        = dom_der;
            var_der.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_der: n_var = {}", n_var);
                println!( "index, bool" );
                for j in 0 .. self.var.bool_all.len() {
                    println!( "{}, {:?}", j, self.var.bool_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "index, dyp_all" );
                for j in 0 .. n_dyp {
                    println!( "{}, {}", j, dyp_all[j] );
                }
                println!( "var_index, var_dom, dom_der" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}, {}", j, var_all[j], var_der[j] );
                }
                println!( "var_index, var_all, var_der, op_name, arg" );
            }
            //
            // var_der
            for op_index in 0 .. self.var.id_all.len() {
                let op_id    = self.var.id_all[op_index] as usize;
                let start    = self.var.arg_start[op_index] as usize;
                let end      = self.var.arg_start[op_index + 1] as usize;
                let arg      = &self.var.arg_all[start .. end];
                let arg_type = &self.var.arg_type_all[start .. end];
                let res      = self.var.n_dom + op_index;
                let forward_der = op_info_vec[op_id].[< forward_der_ $suffix >];
                forward_der(
                    &dyp_all,
                    &var_all,
                    &mut var_der,
                    &self.cop,
                    &self.var.bool_all,
                    arg,
                    arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {}, {:?}",
                        res, var_all[res], var_der[res], name, arg
                    );
                }
            }
            if trace {
                println!( "rng_index, var_index, con_index" );
                for i in 0 .. self.rng_ad_type.len() {
                    let index = self.rng_index[i] as usize;
                    if self.rng_ad_type[i].is_variable() {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_der" );
            }
            let mut range_der : Vec<$E> = Vec::new();
            for i in 0 .. self.rng_ad_type.len() {
                let index = self.rng_index[i] as usize;
                if self.rng_ad_type[i].is_variable() {
                    range_der.push( var_der[index].clone() );
                } else {
                    range_der.push( zero_e.clone() );
                }
            }
            range_der
        }
    }
} }
//
impl<V> ADfn<V> where
V : Clone + std::fmt::Display + GlobalOpInfoVec + FConst + ThisThreadTape,
{   //
    // forward_der
    forward_der!( value, V );
    forward_der!( ad,    AD::<V> );
}
