// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Implements [ADfn] subgraph derivative method.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
//
use crate::{
    AD,
    ADfn,
    IndexT,
    FloatCore,
};
use crate::op::info::{
    OpInfo,
    sealed::GlobalOpInfoVec
};
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
    adfn::reverse_der::doc_reverse_der,
};
//
// ---------------------------------------------------------------------------
// sub_der
/// Evaluate the gradient of one component of f using the subgraph
/// for that component.
///
/// The evaluation of one component of f
/// may have many fewer operations than the entire function.
///
/// * See Also : [doc_reverse_der]
///
/// * Syntax :
///   ```text
///     dom_der = f.subgraph_der(dyp_both, &var_both, row_index, trace)
///   ```
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * f :
///   is this [ADfn] object.
///
/// * row_index :
///   is the index in the range vector that are computing the derivative for;
///   row_index < f.rng_len() .
///
/// * trace :
///   If this is true, a trace of the calculation
///   is printed on standard output.
///
/// * dom_der :
///   The return value *dom_der* is the gradient of the *row_index*
///   component of the range of f.
///
/// # Example
/// ```
/// use rustad::{
///     AD,
///     start_recording,
///     stop_recording,
/// };
/// //
/// // V
/// type V = rustad::AzFloat<f32>;
/// //
/// // trace, n, x
/// let trace      = false;
/// let n          = 5;
/// let x          = vec![ V::from(1); n];
/// //
/// // f
/// let (_, ax)    = start_recording(None, x);
/// let mut ay : Vec< AD<V> > = Vec::new();
/// for i in 0 .. (n-1) {
///     ay.push( &ax[i] * &ax[i+1] );
/// }
/// let f          = stop_recording(ay);
///
/// // x, row_index
/// let x  : Vec<V> = (0 .. n).map( |j| V::from(j+1) ).collect();
/// let row_index = n / 2;
/// //
/// // dom_der
/// let (_y, varboth) = f.forward_var_value(None, x.clone(), trace);
/// let dom_der       = f.subgraph_der_value(None, &varboth, row_index, trace);
/// //
/// // check
/// for j in 1 .. n {
///     if j == row_index {
///         assert_eq!( dom_der[j], x[j+1] );
///     } else if j == row_index + 1 {
///         assert_eq!( dom_der[j], x[j-1] );
///     } else {
///         assert_eq!( dom_der[j], V::from(0) );
///     }
/// }
/// ```
pub fn doc_subgraph_der() {}
/// Create the subgraph_der functions
///
/// * suffix : is either `value` or `ad` ;
/// * E      : see [doc_generic_e] .
///
/// If *suffix* is `value` , *E must be be the value type *V* .
/// If *suffix* is `ad` , *E must be be the type `AD<V>` .
///
/// See [doc_subgraph_der]
macro_rules! subgraph_der{ ($suffix:ident,$V:ident,$E:ty) => {paste::paste! {
    #[doc = concat!(
        "`", stringify!($E), "` evaluation of of subgraph derivatives; ",
        "see [doc_subgraph_der]",
    )]
    //
    pub fn [< subgraph_der_ $suffix >] (
        &self,
        dyp_both  : Option< &Vec<$E> >  ,
        var_both  : &Vec<$E>            ,
        row_index : usize               ,
        trace     : bool                ,
    ) -> Vec<$E>
    {   //
        // dyp_both
        let dyp_both : &Vec<$E> = if dyp_both.is_none() {
            &Vec::new()
        } else {
            dyp_both.unwrap()
        };
        // var_n_dom
        let var_n_dom = self.var_dom_len();
        //
        // zero_e, one_e
        let zero_e      : $E  = FloatCore::zero();
        let one_e       : $E  = FloatCore::one();
        //
        // var_der
        let n_var             = self.var_len();
        let mut var_der       = vec![ zero_e; n_var ];
        //
        // op_info_vec
        let op_info_vec : &Vec< OpInfo<$V> >  = GlobalOpInfoVec::get();
        //
        // rng_ad_type, rng_index, n_range
        let rng_ad_type       = &self.rng_ad_type;
        let rng_index         = &self.rng_index;
        let n_range           = rng_ad_type.len();
        //
        if trace {
            println!("Begin Trace: subgraph_der");
            println!(" var_n_dom = {}, n_range = {}, row_index = {}",
                var_n_dom, n_range, row_index
        ); }
        //
        // var_index_stack, var_index
        let mut var_index_stack : Vec<IndexT> = Vec::new();
        if rng_ad_type[row_index].is_variable() {
            let var_index               = rng_index[row_index];
            var_der[var_index as usize] = one_e;
            var_index_stack.push( var_index );
        }
        while ! var_index_stack.is_empty() {
            let var_index = var_index_stack.pop().unwrap() as usize;
            if var_n_dom <= var_index  {
                //
                // op_index
                let op_index = var_index - var_n_dom;
                //
                // var_der
                let op_id     = self.var.id_all[op_index] as usize;
                let start     = self.var.arg_start[op_index] as usize;
                let end       = self.var.arg_start[op_index + 1] as usize;
                let arg       = &self.var.arg_all[start .. end];
                let arg_type  = &self.var.arg_type_all[start .. end];
                let res       = self.var.n_dom + op_index;
                let reverse_1 = op_info_vec[op_id].[< reverse_der_ $suffix >];
                reverse_1(
                    &dyp_both,
                    &var_both,
                    &mut var_der,
                    &self.cop,
                    &self.var.flag_all,
                    arg,
                    arg_type,
                    res
                );
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!( "{}, {}, {}, {}, {:?}",
                        res, var_both[res], var_der[res], name, arg
                    );
                }
                //
                // var_index_stack
                for i in 0 .. arg.len() {
                    if arg_type[i].is_variable() {
                        var_index_stack.push( arg[i] );
                    }
                }
            }
        }
        if trace {
            println!( "var_index, var_dom, dom_der" );
            for j in 0 .. self.var.n_dom {
                println!( "{}, {}, {}", j, var_both[j], var_der[j] );
            }
            println!( "End Trace: sub_sparsity");
        }
        //
        // domain_der
        let mut domain_der = var_der;
        domain_der.truncate(self.var.n_dom);
        domain_der.shrink_to_fit();
        domain_der
    }
}}}
//
impl<V> ADfn<V> where
V : Clone + std::fmt::Display + GlobalOpInfoVec + FloatCore + ThisThreadTape,
{   //
    // subgraph_der
    subgraph_der!( value, V, V );
    subgraph_der!( ad,    V, AD::<V> );
}
