// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] forward_dyp method (compute dynamic parameters)
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    AD,
    ADfn,
};
use crate::op::info::GlobalOpInfoVec;
use crate::adfn::eval_from::eval_from_f32;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// forward_dyp
/// Compute the dependent dynamic parameters.
///
/// * Syntax :
/// ```text
///     dyp_zero = f.forward_dyp_value(dyp_dom, trace)
///     dyp_zero = f.forward_dyp_ad(dyp_dom, trace)
/// ```
/// * Prototype :
/// see [ADfn::forward_dyp_value] and [ADfn::forward_dyp_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f : is an [ADfn] object.
///
/// * trace :
/// if true, a trace of the calculation is printed on stdout.
///
/// * dyp_dom :
/// specifies the domain space dynamic parameter values.
///
/// * dyp_zero :
/// The return contains the dependent dynamic parameters
/// in an unspecified manner. Even the type of dyp_zero is unspecified; i.e.,
/// it may change in the future. 
/// This vector is used as an input to other *f* member functions.
/// member functions.
///
pub fn doc_forward_dyp() { }
//
/// Create the member function that evaluates the dependent dynamic parameters.
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
/// See [doc_forward_dyp]
macro_rules! forward_dyp {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!( " `", stringify!($E), "`",
        " evaluation of dependent dynamic parameters; see [doc_forward_dyp]" ,
        )]
        pub fn [< forward_dyp_ $suffix >] (
            &self,
            dyp_dom     : Vec<$E>   ,
            trace       : bool      ,
        ) -> Vec<$E>
        {   // dyp_dom
            assert_eq!(
                dyp_dom.len(), self.dyp.n_dom,
                "f.forward_dyp: dyp_dom  vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = &*GlobalOpInfoVec::get();
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            // dyp_zero
            let nan_e  : $E  = eval_from_f32!($suffix, $V,  f32::NAN);
            let mut dyp_zero = vec![ nan_e; n_dyp ];
            for j in 0 .. self.dyp.n_dom {
                dyp_zero[j] = dyp_dom[j].clone();
            }
            //
            if trace {
                println!( "Begin Trace: forward_dyp: n_dyp = {}", n_dyp);
                println!( "index, flag" );
                for j in 0 .. self.dyp.flag.len() {
                    println!( "{}, {}", j, self.dyp.flag[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "dyp_index, dyp_dom" );
                for j in 0 .. self.dyp.n_dom {
                    println!( "{}, {}", j, dyp_dom[j] );
                }
                println!( "dyp_index, dyp_zero, op_name, arg, arg_cop" );
            }
            //
            // dyp_zero
            for op_index in 0 .. self.dyp.id_seq.len() {
                let op_id   = self.dyp.id_seq[op_index] as usize;
                let start   = self.dyp.arg_seq[op_index] as usize;
                let end     = self.dyp.arg_seq[op_index + 1] as usize;
                let arg     = &self.dyp.arg_all[start .. end];
                let arg_cop = &self.dyp.arg_cop[start .. end];
                let res     = self.dyp.n_dom + op_index;
                let forward_dyp = op_info_vec[op_id].[< forward_dyp_ $suffix >];
                forward_dyp(
                    &mut dyp_zero,
                    &self.cop,
                    &self.dyp.flag,
                    &arg,
                    &arg_cop,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}, {:?}",
                        res, dyp_zero[res], name, arg, arg_cop
                    );
                }
            }
            dyp_zero
        }
    }
} }
//
impl<V> ADfn<V>
where
    V     : From<f32> + Clone + std::fmt::Display + GlobalOpInfoVec,
{   //
    // forward_dyp
    forward_dyp!( value, V, V );
    forward_dyp!( ad,    V, AD::<V> );
}

#[cfg(test)]
mod tests {
    use crate::{
        ad_from_value,
        start_recording_both,
        stop_recording,
    };
    //
    #[test]
    fn test_forward_dyp() {
        //
        // V
        type V = f32;
        //
        // f
        let np         = 3;
        let nx         = 1;
        let p : Vec<V> = vec![1.0 as V; np ];
        let x : Vec<V> = vec![1.0 as V; nx ];
        let (ap, ax)   = start_recording_both(p.clone(), x.clone());

        let mut asum   = ad_from_value( V::from(0.0) );
        for j in 0 .. np {
            asum += &ap[j];
        }
        let ay = vec![ &ax[0] * &asum ];
        let f  = stop_recording(ay);
        //
        // dyp_zero
        let trace = false;
        let dyp_zero = f.forward_dyp_value(p.clone(), trace);
        //
        assert_eq!( dyp_zero.len(), 2 * np );
        let mut sum = 0.0 as V;
        for j in 0 .. np {
            sum += p[j];
            assert_eq!( dyp_zero[j], p[j] );
            assert_eq!( dyp_zero[np + j], sum );
        }
    }
}
