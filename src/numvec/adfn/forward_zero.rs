// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] methods that compute function values.
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
use crate::numvec::doc_generic_v;
#[cfg(doc)]
use crate::numvec::doc_generic_e;
//
// -----------------------------------------------------------------------
// forward_zero
/// Zero order forward mode evaluation; i.e., function values.
///
/// * Syntax :
/// ```text
///     (range_zero, var_zero) = f.forward_zero_value(domain_zero, trace)
///     (range_zero, var_zero) = f.forward_zero_ad(domain_zero, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_zero_value] and [ADfn::forward_zero_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f :
/// is an [ADfn] object.
///
/// * domain_zero :
/// specifies the domain space variable values.
///
/// * trace :
/// if true, a trace of the operatiopn sequence is printed on stdout.
///
/// * range_zero :
/// The first return
/// is the range vector corresponding to the domain space variable values;
/// i.e., the value of the function correspdong the operation sequence in f.
///
/// * var_zero :
/// The second return
/// is the value for all the variables in the operation sequence.
/// This is used as an input when computing derivatives.
///
/// # Example
/// Computing function values using forward_zero :
/// ```
/// use rustad::numvec::start_recording;
/// use rustad::numvec::stop_recording;
/// use rustad::numvec::AD;
/// //
/// // V
/// type V = f32;
/// //
/// // f
/// // f(x) = x[0] + ... + x[nx-1]
/// let x        : Vec<V> = vec![ 1.0, 1.0, 1.0 ];
/// let ax                = start_recording(x);
/// let mut asum          = AD::from( V::from(0.0) );
/// for j in 0 .. ax.len() {
///     asum += &ax[j];
/// }
/// let ay = vec![ asum ];
/// let f  = stop_recording(ay);
/// //
/// // y
/// // y[0] = f(x)
/// let trace           = true;
/// let x      : Vec<V> = vec![ 1.0, 2.0, 3.0 ];
/// let (y, v)          = f.forward_zero_value(x, trace);
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
            domain_zero : Vec<$E> ,
            trace       : bool    ,
        ) -> ( Vec<$E> , Vec<$E> )
        {
            assert_eq!(
                domain_zero.len(), self.n_domain,
                "f.forward_zero: domain vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // var_zero
            let nan_v         : $V = f32::NAN.into();
            let nan_e         : $E = nan_v.into();
            let mut var_zero       = domain_zero;
            var_zero.resize( self.n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_zero: n_var = {}", self.n_var);
                println!( "index, flag" );
                for j in 0 .. self.flag_all.len() {
                    println!( "{}, {}", j, self.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.con_all.len() {
                    println!( "{}, {}", j, self.con_all[j] );
                }
                println!( "var_index, domain_zero" );
                for j in 0 .. self.n_domain {
                    println!( "{}, {}", j, var_zero[j] );
                }
                println!( "var_index, var_zero, op_name, arg" );
            }
            for op_index in 0 .. self.id_all.len() {
                let op_id = self.id_all[op_index] as usize;
                let start = self.op2arg[op_index] as usize;
                let end   = self.op2arg[op_index + 1] as usize;
                let arg   = &self.arg_all[start .. end];
                let res   = self.n_domain + op_index;
                let forward_0 = op_info_vec[op_id].[< forward_0_ $suffix >];
                forward_0(&mut var_zero,
                    &self.con_all, &self.flag_all, &arg, res
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
                for i in 0 .. self.range_is_var.len() {
                    let index = self.range2tape_index[i] as usize;
                    if self.range_is_var[i] {
                        println!( "{}, {}, ----", i, index);
                    } else {
                        println!( "{}, ---- ,{}", i, index);
                    }
                }
                println!( "End Trace: forward_zero" );
            }
            let mut range_zero : Vec<$E> = Vec::new();
            for i in 0 .. self.range_is_var.len() {
                let index = self.range2tape_index[i] as usize;
                if self.range_is_var[i] {
                    range_zero.push( var_zero[index].clone() );
                } else {
                    let constant = self.con_all[index].clone();
                    range_zero.push( constant.into() );
                }
            }
            ( range_zero, var_zero )
        }
    } } }
    //
    impl<V> ADfn<V>
    where
        V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
        AD<V> : From<V> ,
    {   //
        // forward_zero
        forward_zero!( value, V, V );
        forward_zero!( ad,    V, AD::<V> );
    }
