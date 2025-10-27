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
// -----------------------------------------------------------------------
// reverse_one
/// First order reverse mode evaluation; i.e., partial derivatives.
///
/// * Syntax :
/// ```text
///     domain_one = f.reverse_one_value(&var_zero, range_one, trace)
///     domain_one = f.reverse_one_ad(   &var_zero, range_one, trace)
/// ```
///
/// * Prototype :
/// see [ADfn::reverse_one_value] and [ADfn::reverse_one_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// ## f
/// is an [ADfn] object.
///
/// ## var_zero
/// must be the *var_zero* computed by a previous call to forward_zero.
///
/// ## range_one
/// specifies the range space weights that define the scalar function
/// for this reverse mode calculation.
///
/// ## trace
/// if true, a trace of the operations is printed on stdout.
///
///
/// ## domain_one
/// The return value is the partial derivative
/// ```text
///     domain_one = range_one * f'(domain_zero)
/// ```
/// Here `f'` is the derivative of the function and
/// [domain_zero](doc_forward_zero#domain_zero) is its value in the call to
/// forward_zero that created the *var_zero* .
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
/// let mut v0 : Vec<V> = Vec::new();
/// let y0              = f.forward_zero_value(&mut v0, x0, trace);
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
            var_zero    : &Vec<$E>  ,
            range_one   : Vec<$E>   ,
            trace       : bool      ,
        ) -> Vec<$E>
        {
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            assert_eq!(
                range_one.len(), self.range2ad_type.len(),
                "f.reverse_one: range vector length does not match f"
            );
            assert_eq!(
                 var_zero.len(), n_var,
                "f.reverse_one:  var_zero does not have the proper length"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // zero_e
            let zero_e : $E = eval_from_f32!($suffix, $V, 0 as f32);
            //
            // var_one
            let mut var_one       = vec![ zero_e; n_var ];
            let mut mut_range_one = range_one;
            for i in (0 .. self.range2ad_type.len()).rev() {
                let y_i = mut_range_one.pop().unwrap();
                if self.range2ad_type[i] == ADType::Variable {
                    let index = self.range2index[i] as usize;
                    var_one[index] = y_i;
                }
            }
            //
            if trace {
                println!( "Begin Trace: reverse_one: n_var = {}", n_var);
                println!( "index, flag" );
                for j in 0 .. self.var.flag.len() {
                    println!( "{}, {}", j, self.var.flag[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "var_index, range_one" );
                for i in 0 .. self.range2ad_type.len() {
                    if self.range2ad_type[i] == ADType::Variable {
                        let index = self.range2index[i] as usize;
                        println!( "{}, {}", index,  var_one[index] );
                    }
                }
                println!( "var_index, var_zero, var_one, op_name, arg" );
            }
            //
            // var_one
            for op_index in ( 0 .. self.var.id_seq.len() ).rev() {
                let op_id = self.var.id_seq[op_index] as usize;
                let start = self.var.arg_seq[op_index] as usize;
                let end   = self.var.arg_seq[op_index + 1] as usize;
                let arg   = &self.var.arg_all[start .. end];
                let res   = self.var.n_dom + op_index;
                let reverse_1 = op_info_vec[op_id].[< reverse_1_ $suffix >];
                reverse_1(
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
                println!( "End Trace: reverse_one" );
            }
            //
            // domain_one
            let nan_e  : $E    = eval_from_f32!($suffix, $V,  f32::NAN);
            let mut domain_one = var_one;
            domain_one.resize(self.var.n_dom, nan_e);
            domain_one.shrink_to_fit();
            domain_one
        }
    } } }
    //
    impl<V> ADfn<V>
    where
        V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
    {   //
        // reverse_one
        reverse_one!( value, V, V );
        reverse_one!( ad,    V, AD::<V> );
    }
