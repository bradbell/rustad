// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_var method (function values).
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
};
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
// forward_var
/// Zero order forward mode evaluation; i.e., function values.
///
/// * Syntax :
/// ```text
///     (range_zero, var_zero) = f.forward_var_value(&dyp_zero, var_dom, trace)
///     (range_zero, var_zero)  = f.forward_var_ad(&dyp_zero, var_dom, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_var_value] and [ADfn::forward_var_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
/// * f : is an [ADfn] object.
///
/// * dyp_zero :
/// is the dynamic parameter vector in the following order:
/// the domain dynamic parameters followed by the dependent dynamic parameters.
/// This is normally computed by
/// [forward_dyp](crate::adfn::forward_dyp::doc_forward_dyp) .
/// In the special case where there are no dynamic parameters,
/// *dyp_zero* can be set to the empty vector
/// ( it is not necessary to call `forward_dyp` ).
///
/// * var_dom :
/// This the the domain variable values.
///
/// * trace :
/// if true, a trace of the calculation is printed on stdout.
///
/// * range_zero :
/// is the range vector corresponding to the
/// domain variable and parameter values;
/// i.e., the value of the function correspdong the operation sequence in f.
/// Note that a range space component may be a
/// variable, a dynamic parameter, or a constant parameter;
/// see [ADfn::range_ad_type] .
///
/// * var_zero :
/// is the variable vector in the following order:
/// the domain variables followed by the dependent variables.
/// Note that *dym_var* gets moved to the beginning of *var_zero* .
///
/// # Example
/// Computing function values using forward_var :
/// ```
/// use rustad::start_recording_both;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::ad_from_value;
/// //
/// // V
/// type V = f32;
///
/// // np, nx
/// let np = 2;
/// let nx = 3;
/// //
/// // f
/// // f(p, x) = (p[0] + ... + p[np-1]) * (x[0] + ... + x[nx-1])
/// let p        : Vec<V> = vec![ 1.0; np];
/// let x        : Vec<V> = vec![ 1.0; nx];
/// let (ap, ax)          = start_recording_both(p, x);
/// let mut ap_sum        = ad_from_value( V::from(0.0) );
/// for j in 0 .. np {
///     ap_sum += &ap[j];
/// }
/// let mut ax_sum        = ad_from_value( V::from(0.0) );
/// for j in 0 .. nx {
///     ax_sum += &ax[j];
/// }
/// let ay = vec![ &ap_sum * &ax_sum ];
/// let f  = stop_recording(ay);
/// //
/// // trace, p, x
/// let trace           = false;
/// let mut p : Vec<V> = Vec::new();
/// for j in 1 .. np+1 {
///     p.push(j as V);
/// }
/// let mut x : Vec<V> = Vec::new();
/// for j in 1 .. nx+1 {
///     x.push(j as V);
/// }
/// //
/// // y = f(p, x)
/// let dyp_zero      = f.forward_dyp_value(p.clone(), trace);
/// let (y, var_zero) = f.forward_var_value(&dyp_zero, x.clone(), trace);
/// //
/// // check
/// let p_sum = ( np * (np + 1) ) / 2;
/// let x_sum = ( nx * (nx + 1) ) / 2;
/// //
/// assert_eq!( y[0] , (p_sum * x_sum) as V );
/// ```
///
pub fn doc_forward_var() { }
//
/// Create the zero order forward mode member functions.
///
/// * suffix : is either `value` or `ad` ;
///
/// * V : see [doc_generic_v]
///
/// * E : see [doc_generic_e] .
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_forward_var]
macro_rules! forward_var {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "`",
            " evaluation of dependent variables; see [doc_forward_var]",
        )]
        pub fn [< forward_var_ $suffix >] (
            &self,
            dyp_zero    : &Vec<$E>     ,
            var_dom     : Vec<$E>      ,
            trace       : bool         ,
        ) -> ( Vec<$E> , Vec<$E> )
        {
            assert_eq!(
                var_dom.len(), self.var.n_dom,
                "f.forward_var: var_dom vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            //
            // n_var
            let n_var = self.var.n_dom + self.var.n_dep;
            //
            // var_zero
            let nan_e  : $E  = eval_from_f32!($suffix, $V,  f32::NAN);
            let mut var_zero = var_dom;
            var_zero.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_var_{}", stringify!($suffix) );
                println!( "index, flag" );
                for j in 0 .. self.var.flag.len() {
                    println!( "{}, {}", j, self.var.flag[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "index, dyp_zero" );
                for j in 0 .. n_dyp {
                    println!( "{}, {}", j, dyp_zero[j] );
                }
                println!( "index, var_dom" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}", j, var_zero[j] );
                }
                println!( "index, var_zero, op_name, arg, arg_type" );
            }
            for op_index in 0 .. self.var.id_seq.len() {
                let op_id     = self.var.id_seq[op_index] as usize;
                let start     = self.var.arg_seq[op_index] as usize;
                let end       = self.var.arg_seq[op_index + 1] as usize;
                let arg       = &self.var.arg_all[start .. end];
                let arg_type  = &self.var.arg_type[start .. end];
                let res       = self.var.n_dom + op_index;
                let forward_var = op_info_vec[op_id].[< forward_var_ $suffix >];
                //
                forward_var(
                    &dyp_zero,
                    &mut var_zero,
                    &self.cop,
                    &self.var.flag,
                    &arg,
                    &arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}, {:?}",
                        res, var_zero[res], name, arg, arg_type
                    );
                }
            }
            //
            // n_range
            let n_range = self.range_ad_type.len();
            //
            // range_zero
            if trace {
                println!( "range_index, ad_type, index" );
                for i in 0 .. n_range {
                    let ad_type = self.range_ad_type[i].clone();
                    let index   = self.range_index[i] as usize;
                    println!( "{}, {:?}, {}", i, ad_type, index);
                }
                println!( "End Trace: forward_var" );
            }
            let mut range_zero : Vec<$E> = Vec::with_capacity(n_range);
            for i in 0 .. n_range {
                let ad_type = self.range_ad_type[i].clone();
                let index   = self.range_index[i] as usize;
                match ad_type {
                    ADType::Variable =>
                        range_zero.push( var_zero[index].clone() )
                    ,
                    ADType::DynamicP =>
                        range_zero.push( dyp_zero[index].clone() )
                    ,
                    ADType::ConstantP => {
                        let cop_v = self.cop[index].clone();
                        let cop_e = eval_from_value!($suffix, $V, cop_v);
                        range_zero.push( cop_e )
                    },
                    ADType::NoType => {
                        panic!( "forward_var: ADType::NoTYpe not expected" );
                    },
                }
            }
            ( range_zero, var_zero )
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // forward_var
    forward_var!( value, V, V );
    forward_var!( ad,    V, AD::<V> );
}
