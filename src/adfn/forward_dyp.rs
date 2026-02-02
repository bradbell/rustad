// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
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
    FloatCore,
};
use crate::op::info::sealed::GlobalOpInfoVec;
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
///   ```text
///     dyp_both = f.forward_dyp_value(dyp_dom, trace)
///     dyp_both = f.forward_dyp_ad(dyp_dom, trace)
///   ```
/// * Prototype :
///   see [ADfn::forward_dyp_value] and [ADfn::forward_dyp_ad]
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f : is an [ADfn] object.
///
/// * trace :
///   if true, a trace of the calculation is printed on stdout.
///
/// * dyp_dom :
///   specifies the domain space dynamic parameter values.
///
/// * dyp_both :
///   is both the dynamic parameter sub-vectors in the following order:
///   the domain dynamic parameters followed by the dependent dynamic parameters.
///   Note that *dyp_dom* gets moved to the beginning of *dyp_both* .
///
pub fn doc_forward_dyp() { }
//
/// Create the member function that evaluates the dependent dynamic parameters.
///
/// * suffix :
///   is either `value` or `ad` ;
///
/// * V : see [doc_generic_v]
///
/// * E : see [doc_generic_e] .
///   If *suffix* is `value` , *E must be be the value type *V* .
///   If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_forward_dyp]
macro_rules! forward_dyp {
    ( $suffix:ident, $E:ty ) => { paste::paste! {
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
                "f.forward_dyp: dyp_dom vector length does not match f"
            );
            //
            // op_info_vec
            let op_info_vec = GlobalOpInfoVec::get();
            //
            // n_dyp
            let n_dyp = self.dyp.n_dom + self.dyp.n_dep;
            //
            // dyp_both
            let nan_e  : $E  = FloatCore::nan();
            let mut dyp_both = dyp_dom;
            dyp_both.resize(n_dyp, nan_e );
            //
            if trace {
                println!( "Begin Trace: forward_dyp_{}", stringify!($suffix) );
                println!( "index, flag" );
                for j in 0 .. self.dyp.flag_all.len() {
                    println!( "{}, {:?}", j, self.dyp.flag_all[j] );
                }
                println!( "index, constant" );
                for j in 0 .. self.cop.len() {
                    println!( "{}, {}", j, self.cop[j] );
                }
                println!( "dyp_index, dyp_dom" );
                for j in 0 .. self.dyp.n_dom {
                    println!( "{}, {}", j, dyp_both[j] );
                }
                println!( "index, dyp_both, op_name, arg, arg_type" );
            }
            //
            // dyp_both
            for op_index in 0 .. self.dyp.id_all.len() {
                let op_id    = self.dyp.id_all[op_index] as usize;
                let start    = self.dyp.arg_start[op_index] as usize;
                let end      = self.dyp.arg_start[op_index + 1] as usize;
                let arg      = &self.dyp.arg_all[start .. end];
                let arg_type = &self.dyp.arg_type_all[start .. end];
                let res      = self.dyp.n_dom + op_index;
                let forward_dyp = op_info_vec[op_id].[< forward_dyp_ $suffix >];
                //
                forward_dyp(
                    &mut dyp_both,
                    &self.cop,
                    &self.dyp.flag_all,
                    arg,
                    arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {:?}, {:?}",
                        res, dyp_both[res], name, arg, arg_type
                    );
                }
            }
            if trace {
                println!("End Trace: forward_dyp_{}", stringify!($suffix));
            }
            dyp_both
        }
    }
} }
//
impl<V> ADfn<V>
where
    V : Clone + std::fmt::Display + GlobalOpInfoVec + FloatCore,
{   //
    // forward_dyp
    forward_dyp!( value, V );
    forward_dyp!( ad,    AD::<V> );
}

#[cfg(test)]
mod tests {
    use crate::{
        AzFloat,
        ad_from_value,
        start_recording,
        stop_recording,
    };
    //
    #[test]
    fn test_forward_dyp() {
        //
        // V
        type V = AzFloat<f32>;
        //
        // f
        let np         = 3;
        let nx         = 1;
        let p : Vec<V> = vec![ V::from(1.0); np ];
        let x : Vec<V> = vec![ V::from(1.0 ); nx ];
        let (ap, ax)   = start_recording( Some(p.clone()), x.clone());
        //
        // asum
        // The first addition adds the constants zero and so is not recorded
        let mut asum   = ad_from_value( V::from(0.0) );
        for j in 0 .. np {
            asum += &ap[j];
        }
        //
        // f
        let ay = vec![ &ax[0] * &asum ];
        let f  = stop_recording(ay);
        //
        // dyp_both
        let trace = false;
        let dyp_both = f.forward_dyp_value(p.clone(), trace);
        //
        assert_eq!( dyp_both.len(), 2 * np - 1 );
        for j in 0 .. np {
            assert_eq!( dyp_both[j], p[j] );
        }
        let mut sum = p[0];
        for j in 1 .. np {
            sum += &p[j];
            assert_eq!( dyp_both[np + j - 1], sum );
        }
    }
}
