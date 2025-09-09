// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] methods that compute directional derivatives.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::numvec::AD;
use crate::numvec::ADfn;
use crate::numvec::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::numvec::{
    doc_generic_v,
    doc_generic_e,
};
#[cfg(doc)]
use crate::numvec::adfn::forward_zero::doc_forward_zero;
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
/// use rustad::numvec::start_recording;
/// use rustad::numvec::stop_recording;
/// use rustad::numvec::AD;
///
/// // V
/// type V = f64;
/// //
/// // f
/// // f(x) = x[0] * x[1] * x[2]
/// let x        : Vec<V> = vec![ 1.0, 1.0, 1.0 ];
/// let ax                  = start_recording(x);
/// let mut aprod           = AD::from( V::from(1.0) );
/// for j in 0 .. ax.len() {
///     aprod *= &ax[j];
/// }
/// let ay = vec![ aprod ];
/// let f  = stop_recording(ay);
/// //
/// // y
/// // y[0] = f(x)
/// let trace           = true;
/// let x0     : Vec<V> = vec![ 4.0, 5.0, 6.0 ];
/// let mut c0 : Vec<V> = Vec::new();
/// let y0              = f.forward_zero_value(&mut c0, x0, trace);
/// let x1     : Vec<V> = vec![ 1.0, 0.0, 0.0 ];
/// let y1              = f.forward_one_value(&mut c0, x1,  trace);
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
            assert_eq!(
                domain_one.len(), self.n_domain,
                "f.forward_one: domain vector length does not match f"
            );
            assert_eq!(
                var_zero.len(), self.n_var,
                "f.forward_one: var_zero does not have the correct length"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_v        : $V = 0f32.into();
            let zero_e        : $E = zero_v.into();
            //
            // var_one
            let nan_v         : $V = f32::NAN.into();
            let nan_e         : $E = nan_v.into();
            let mut var_one        = domain_one;
            var_one.resize( self.n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_one: n_var = {}", self.n_var);
                println!( "index, flag" );
                for j in 0 .. self.flag_all.len() {
                    println!( "{}, {}", j, self.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "var_index, domain_zero, domain_one" );
                for j in 0 .. self.n_domain {
                    println!( "{}, {}, {}", j, var_zero[j], var_one[j] );
                }
                println!( "var_index, var_zero, var_one, op_name, arg" );
            }
            //
            // var_one
            for op_index in 0 .. self.id_all.len() {
                let op_id = self.id_all[op_index] as usize;
                let start = self.op2arg[op_index] as usize;
                let end   = self.op2arg[op_index + 1] as usize;
                let arg   = &self.arg_all[start .. end];
                let res   = self.n_domain + op_index;
                let forward_1 = op_info_vec[op_id].[< forward_1_ $suffix >];
                forward_1(
                    &var_zero,
                    &mut var_one,
                    &self.con_all,
                    &self.flag_all,
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
                for i in 0 .. self.range_is_var.len() {
                    let index = self.range2tape_index[i] as usize;
                    if self.range_is_var[i] {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_one" );
            }
            let mut range_one : Vec<$E> = Vec::new();
            for i in 0 .. self.range_is_var.len() {
                let index = self.range2tape_index[i] as usize;
                if self.range_is_var[i] {
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
        AD<V> : From<V> ,
    {   //
        // forward_one
        forward_one!( value, V, V );
        forward_one!( ad,    V, AD::<V> );
    }
