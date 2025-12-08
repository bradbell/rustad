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
use crate::ad::ADType;
use crate::{
    AD,
    ADfn,
};
use crate::op::id::CALL_OP;
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::adfn::eval_from::eval_from_f32;
use crate::adfn::eval_from::eval_from_value;
use crate::op::call::extract_call_info;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// forward_var
/// Zero order forward mode variable evaluation with dynamic parameters.
///
/// * Syntax :
/// ```text
///     (range, var_both) = f.forward_var_value(&dyp_both, var_dom, trace)
///     (range, var_both)  = f.forward_var_ad(&dyp_both, var_dom, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_var_value] and [ADfn::forward_var_ad]
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
/// * var_dom :
/// This the the domain variable values.
///
/// * trace :
/// if true, a trace of the calculation is printed on stdout.
///
/// * range :
/// is the range vector corresponding to the
/// domain variable and parameter values;
/// i.e., the value of the function correspdong the operation sequence in f.
/// Note that a range space component may be a
/// variable, a dynamic parameter, or a constant parameter.
///
/// * var_both :
/// is both the variable sub-vectors in the following order:
/// the domain variables followed by the dependent variables.
/// Note that *var_dom* gets moved to the beginning of *var_both* .
///
/// # Example
/// Computing function values using forward_var :
/// ```
/// use rustad::start_recording_dyp_var;
/// use rustad::stop_recording;
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// //
/// // V
/// type V = rustad::AzFloat<f32>;
///
/// // np, nx
/// let np = 2;
/// let nx = 3;
/// //
/// // f
/// // f(p, x) = (p[0] + ... + p[np-1]) * (x[0] + ... + x[nx-1])
/// let p                 = vec![ V::from(1.0); np];
/// let x                 = vec![ V::from(1.0); nx];
/// let (ap, ax)          = start_recording_dyp_var(p, x);
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
///     p.push(V::from(j));
/// }
/// let mut x : Vec<V> = Vec::new();
/// for j in 1 .. nx+1 {
///     x.push(V::from(j));
/// }
/// //
/// // y = f(p, x)
/// let dyp_both      = f.forward_dyp_value(p.clone(), trace);
/// let (y, var_both) = f.forward_var_value(&dyp_both, x.clone(), trace);
/// //
/// // check
/// let p_sum = ( np * (np + 1) ) / 2;
/// let x_sum = ( nx * (nx + 1) ) / 2;
/// //
/// assert_eq!( y[0] , V::from(p_sum * x_sum) );
/// ```
///
pub fn doc_forward_var() { }
//
/// Create the zero order forward mode member functions with dynamic parameters.
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
            dyp_both    : &Vec<$E>     ,
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
            // var_both
            let nan_e  : $E  = eval_from_f32!($suffix, $V,  f32::NAN);
            let mut var_both = var_dom;
            var_both.resize( n_var, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_var_{}", stringify!($suffix) );
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
                println!( "index, var_dom" );
                for j in 0 .. self.var.n_dom {
                    println!( "{}, {}", j, var_both[j] );
                }
                println!( "index, var_both, op_name, arg, arg_type" );
            }
            for op_index in 0 .. self.var.id_all.len() {
                let op_id     = self.var.id_all[op_index] as usize;
                let start     = self.var.arg_seq[op_index] as usize;
                let end       = self.var.arg_seq[op_index + 1] as usize;
                let arg       = &self.var.arg_all[start .. end];
                let arg_type  = &self.var.arg_type_all[start .. end];
                let res       = self.var.n_dom + op_index;
                let forward_var = op_info_vec[op_id].[< forward_var_ $suffix >];
                //
                forward_var(
                    &dyp_both,
                    &mut var_both,
                    &self.cop,
                    &self.var.flag_all,
                    &arg,
                    &arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}, {:?}",
                        res, var_both[res], name, arg, arg_type
                    );
                    if op_id as u8 == CALL_OP {
                        let (
                            _atom_id,
                            _call_info,
                            _n_dom,
                            n_rng,
                            _trace,
                            rng_is_dep,
                        ) = extract_call_info(arg, &self.dyp.flag_all);
                        let mut i_dep = 0;
                        for i_rng in 0 .. n_rng {
                            if rng_is_dep[i_rng] { if i_dep > 0 { println!(
                                "{}, {}", res + i_dep, var_both[res + i_dep]
                            ); } }
                            if rng_is_dep[i_rng] {
                                i_dep += 1;
                            }
                        }
                    }
                }
            }
            //
            // n_range
            let n_range = self.rng_ad_type.len();
            //
            // range
            if trace {
                println!( "rng_index, ad_type, index" );
                for i in 0 .. n_range {
                    let ad_type = self.rng_ad_type[i].clone();
                    let index   = self.rng_index[i] as usize;
                    println!( "{}, {:?}, {}", i, ad_type, index);
                }
                println!( "End Trace: forward_var" );
            }
            let mut range : Vec<$E> = Vec::with_capacity(n_range);
            for i in 0 .. n_range {
                let ad_type = self.rng_ad_type[i].clone();
                let index   = self.rng_index[i] as usize;
                match ad_type {
                    ADType::Variable =>
                        range.push( var_both[index].clone() )
                    ,
                    ADType::DynamicP =>
                        range.push( dyp_both[index].clone() )
                    ,
                    ADType::ConstantP => {
                        let cop_v = self.cop[index].clone();
                        let cop_e = eval_from_value!($suffix, $V, cop_v);
                        range.push( cop_e )
                    },
                    _ => {
                        panic!( "forward_var: and AD type not expected" );
                    },
                }
            }
            ( range, var_both )
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
