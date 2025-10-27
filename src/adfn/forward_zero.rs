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
use crate::AD;
use crate::ADfn;
use crate::ADType;
use crate::op::info::GlobalOpInfoVec;
use crate::adfn::eval_from::eval_from_f32;
use crate::adfn::eval_from::eval_from_value;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode evaluation; i.e., function values.
///
/// * Syntax :
/// ```text
///     range_zero = f.forward_zero_value(&mut var_zero, domain_zero, trace)
///     range_zero = f.forward_zero_ad(   &mut var_zero, domain_zero, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_zero_value] and [ADfn::forward_zero_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// ## f
/// is an [ADfn] object.
///
/// ## var_zero
/// The input value of this vector should have length zero.
/// Upon return it has the zero order forward mode values for all
/// the variables in the operation sequence.
/// This begins with *domain.zero* ; i.e.,
/// ```text
///     var_zero[ 0 .. domain_zero.len() ] == domain_zero
/// ```
/// It may be useful to know this because domain_zero is consumed by
/// this operation.
///
/// ## trace
/// if true, a trace of the calculation is printed on stdout.
///
/// ## range_zero
/// The first return
/// is the range vector corresponding to the domain space variable values;
/// i.e., the value of the function correspdong the operation sequence in f.
///
/// ## domain_zero
/// specifies the domain space variable values.
///
/// # Example
/// Computing function values using forward_zero :
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::ad_from_value;
/// //
/// // V
/// type V = f32;
/// //
/// // f
/// // f(x) = x[0] + ... + x[nx-1]
/// let x        : Vec<V> = vec![ 1.0, 1.0, 1.0 ];
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
/// let x      : Vec<V> = vec![ 1.0, 2.0, 3.0 ];
/// let mut v0 : Vec<V> = Vec::new();
/// let y  = f.forward_zero_value(&mut v0, x, trace);
/// //
/// assert_eq!( y[0] , (1 + 2 + 3) as V );
/// ```
///
pub fn doc_forward_zero() { }
//
/// Create the zero order forward mode member functions.
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
            var_zero    : &mut Vec<$E> ,
            domain_zero : Vec<$E>      ,
            trace       : bool         ,
        ) -> Vec<$E>
        {   assert_eq!(
                var_zero.len(), 0,
                "f.forward_zero: var_zero  does not have length zero"
            );
            assert_eq!(
                domain_zero.len(), self.var.n_dom,
                "f.forward_zero: domain vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            // var_zero
            let nan_e  : $E  = eval_from_f32!($suffix, $V,  f32::NAN);
            *var_zero        = domain_zero;
            var_zero.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_zero: n_var = {}", n_var);
                println!( "index, flag" );
                for j in 0 .. self.var.flag.len() {
                    println!( "{}, {}", j, self.var.flag[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "var_index, domain_zero" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}", j, var_zero[j] );
                }
                println!( "var_index, var_zero, op_name, arg" );
            }
            for op_index in 0 .. self.var.id_seq.len() {
                let op_id = self.var.id_seq[op_index] as usize;
                let start = self.var.arg_seq[op_index] as usize;
                let end   = self.var.arg_seq[op_index + 1] as usize;
                let arg   = &self.var.arg_all[start .. end];
                let res   = self.var.n_dom + op_index;
                let forward_0 = op_info_vec[op_id].[< forward_0_ $suffix >];
                forward_0(var_zero,
                    &self.cop, &self.var.flag, &arg, res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}",
                        res, var_zero[res], name, arg
                    );
                }
            }
            //
            // var_one
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
                println!( "End Trace: forward_zero" );
            }
            let mut range_zero : Vec<$E> = Vec::new();
            for i in 0 .. self.range_ad_type.len() {
                let index = self.range_index[i] as usize;
                if self.range_ad_type[i] == ADType::Variable {
                    range_zero.push( var_zero[index].clone() );
                } else {
                    let constant_v = self.cop[index].clone();
                    let constant_e = eval_from_value!($suffix, $V, constant_v);
                    range_zero.push( constant_e );
                }
            }
            range_zero
        }
    } } }
    //
    impl<V> ADfn<V>
    where
        V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
    {   //
        // forward_zero
        forward_zero!( value, V, V );
        forward_zero!( ad,    V, AD::<V> );
    }
