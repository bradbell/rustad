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
use crate::ADType;
use crate::op::info::GlobalOpInfoVec;
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
// forward_one
/// First order forward mode evaluation; i.e., directional derivatives.
///
/// * Syntax :
/// ```text
///     range_one = f.forward_one_value(&var_zero, domain_one, trace)
///     range_one = f.forward_one_ad(   &var_zero, domain_one, trace)
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
/// ## var_zero
/// must be the *var_zero* computed a the previous call to forward_zero.
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
/// forward_zero that created the *var_zero* .
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
            var_zero    : &Vec<$E>     ,
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
                var_zero.len(), n_var,
                "f.forward_one: var_zero does not have the correct length"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_e        : $E = eval_from_f32!($suffix, $V, f32::NAN);
            //
            // var_one
            let nan_e         : $E = eval_from_f32!($suffix, $V, f32::NAN);
            let mut var_one        = domain_one;
            var_one.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_one: n_var = {}", n_var);
                println!( "index, flag" );
                for j in 0 .. self.var.flag.len() {
                    println!( "{}, {}", j, self.var.flag[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "var_index, domain_zero, domain_one" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}, {}", j, var_zero[j], var_one[j] );
                }
                println!( "var_index, var_zero, var_one, op_name, arg" );
            }
            //
            // var_one
            for op_index in 0 .. self.var.id_seq.len() {
                let op_id = self.var.id_seq[op_index] as usize;
                let start = self.var.arg_seq[op_index] as usize;
                let end   = self.var.arg_seq[op_index + 1] as usize;
                let arg   = &self.var.arg_all[start .. end];
                let res   = self.var.n_dom + op_index;
                let forward_1 = op_info_vec[op_id].[< forward_1_ $suffix >];
                forward_1(
                    &var_zero,
                    &mut var_one,
                    &self.cop,
                    &self.var.flag,
                    &arg,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {}, {:?}",
                        res, var_zero[res], var_one[res], name, arg
                    );
                }
            }
            if trace {
                println!( "range_index, var_index, con_index" );
                for i in 0 .. self.range_ad_type.len() {
                    let index = self.range_index[i] as usize;
                    if self.range_ad_type[i] == ADType::Variable {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_one" );
            }
            let mut range_one : Vec<$E> = Vec::new();
            for i in 0 .. self.range_ad_type.len() {
                let index = self.range_index[i] as usize;
                if self.range_ad_type[i] == ADType::Variable {
                    range_one.push( var_one[index].clone() );
                } else {
                    range_one.push( zero_e.clone() );
                }
            }
            range_one
        }
    } } }
    //
    impl<V> ADfn<V>
    where
        V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
    {   //
        // forward_one
        forward_one!( value, V, V );
        forward_one!( ad,    V, AD::<V> );
    }
