// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Implements [ADfn] sub-graph sparsity method.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
//
use crate::{
    ADfn,
    IndexT,
    SparsityPattern
};
use crate::op::call::call_depend;
use crate::atom::sealed::GlobalAtomCallbackVec;
use crate::atom::AtomCallback;
use crate::op::info::{
    OpInfo,
    sealed::GlobalOpInfoVec
};
use crate::op::id::{
    CALL_OP,
    CALL_RES_OP,
};
//
#[cfg(doc)]
use crate::doc_generic_v;
//
// ---------------------------------------------------------------------------
// ADfn::subsparsity
impl<V> ADfn<V>
where
    V               : GlobalAtomCallbackVec + GlobalOpInfoVec,
    AtomCallback<V> : Clone,
{
    /// Use the subgraph method to compute a Jacobian sparsity pattern.
    ///
    /// * See Also : [ADfn::for_sparsity]
    ///
    /// * Syntax :
    ///   ```text
    ///     (dyp_pattern, var_pattern) = f.sub_sparsity(trace, compute_dyp)
    ///   ```
    ///
    /// * V : see [doc_generic_v]
    ///
    /// * f :
    ///   is this [ADfn] object. The sparsity pattern is for the Jacobian
    ///   of the function defined by the operation sequence stored in f.
    ///
    /// * trace :
    ///   If this is true, a trace of the sparsity calculation
    ///   is printed on standard output.
    ///   Note that in the trace, the cases where *var_index* is less
    ///   that the number of domain variables will end up in the pattern
    ///   with the corresponding row.
    ///
    /// * compute_dyp :
    ///   If this is true (false),
    ///   the dynamic parameter pattern is (is not) computed.
    ///
    /// * dyp_pattern :
    ///   This return is vector of [row, column] pairs.
    ///   Each row (column) is less than the range (dynamic parameter domain)
    ///   dimension for this function.
    ///   If a pair [i, j] does not appear, the range component with index i
    ///   does not depend on the domain dynamic parameter with index j.
    ///
    /// * var_pattern :
    ///   This return is vector of [row, column] pairs.
    ///   Each row (column) is less than the range (variable domain)
    ///   dimension for this function.
    ///   If a pair [i, j] does not appear, the range component
    ///   with index i does not depend on the domain variable with index j.
    ///
    /// ## dependency :
    /// This is a dependency pattern. For example,
    /// if an range variable was equal to the
    /// Heaviside function of a domain variable,
    /// the corresponding pair would be in the sparisty pattern even though
    /// the corresponding derivative is always zero.
    ///
    /// # Example
    /// ```
    /// use rustad::AD;
    /// use rustad::AzFloat;
    /// use rustad::ad_from_value;
    /// use rustad::start_recording;
    /// use rustad::stop_recording;
    /// //
    /// // V
    /// type V = rustad::AzFloat<f32>;
    /// //
    /// // nx, x, ax
    /// let nx = 4;
    /// let x                     = vec![ V::from(2.0); nx];
    /// let (_, ax)               = start_recording(None, x);
    /// //
    /// // ay
    /// let mut ay : Vec< AD<V> > = Vec::new();
    /// ay.push( ad_from_value( V::from(5.0) ) ); // ay[0] is a constant
    /// for j in 1 .. nx {
    ///     ay.push( &ax[j] * &ax[j] );      // ay[j] is a variable
    /// }
    /// //
    /// // f
    /// let f           = stop_recording(ay);
    /// //
    /// // pattern
    /// let trace            = false;
    /// let compute_dyp      = false;
    /// let (_, mut pattern) = f.sub_sparsity(trace, compute_dyp);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), nx - 1 );
    /// for j in 1 .. nx {
    ///     assert_eq!( pattern[j-1], [j, j] );
    /// }
    /// ```
    pub fn sub_sparsity(
        &self, trace : bool, compute_dyp : bool
    ) -> ( SparsityPattern, SparsityPattern )
    {   //
        // op_info_vec
        let op_info_vec : &Vec< OpInfo<V> >  = GlobalOpInfoVec::get();
        //
        // rng_ad_type, rng_index, n_range
        let rng_ad_type       = &self.rng_ad_type;
        let rng_index         = &self.rng_index;
        let n_range           = rng_ad_type.len();
        //
        // var_ : n_dom, n_dep, id_all, arg_start, arg_all, arg_type_all.
        let var_n_dom             = self.var.n_dom;
        let var_n_dep             = self.var.n_dep;
        let var_id_all            = &self.var.id_all;
        let var_arg_start         = &self.var.arg_start;
        let var_arg_all           = &self.var.arg_all;
        let var_arg_type_all      = &self.var.arg_type_all;
        //
        // dyp_ : n_dom, n_dep, id_all, arg_start, arg_all, arg_type_all.
        let dyp_n_dom             = self.dyp.n_dom;
        let dyp_id_all            = &self.dyp.id_all;
        let dyp_arg_start         = &self.dyp.arg_start;
        let dyp_arg_all           = &self.dyp.arg_all;
        let dyp_arg_type_all      = &self.dyp.arg_type_all;
        //
        // var_done, dyp_done
        // initialize all elements as n_var (an invalid variable index)
        let n_var    = var_n_dom + var_n_dep;
        let mut var_done = vec![n_var; n_var];
        let mut dyp_done = vec![n_var; n_var];
        //
        // var_pattern, var_index_stack
        let mut var_pattern     : SparsityPattern   = Vec::new();
        let mut var_index_stack : Vec<IndexT>       = Vec::new();
        //
        // dyp_pattern, dyp_index_stack
        let mut dyp_pattern     : SparsityPattern   = Vec::new();
        let mut dyp_index_stack : Vec<IndexT>       = Vec::new();
        //
        if trace { println!(
            "Begin Trace: sub_sparsity: var_n_dom = {}, n_range = {}",
            var_n_dom, n_range
        ); }
        //
        // atom_depend, cop_depend, dyp_depend, var_depend
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut cop_depend  : Vec<IndexT> = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // row
        // determine the variables that range index row depends on
        for row in 0 .. n_range {
            //
            if trace {
                println!( "row = {}", row );
            }
            //
            // var_index_stack, dyp_index_stack
            // use clear instead of new stack to reduce memory allocation
            var_index_stack.clear();
            dyp_index_stack.clear();
            if rng_ad_type[row].is_variable() {
                var_index_stack.push( rng_index[row] );
            } else if compute_dyp && rng_ad_type[row].is_dynamic() {
                dyp_index_stack.push( rng_index[row] );
            }
            //
            // var_index
            // range[row] depends on this variable
            while ! var_index_stack.is_empty() {
                let var_index = var_index_stack.pop().unwrap() as usize;
                //
                if var_done[var_index] != row {
                    var_done[var_index] = row;
                    if var_index < var_n_dom {
                        if trace {
                            println!( "    var_col = {}", var_index );
                        }
                        //
                        // var_pattern
                        // var_index is a domain variable index
                        var_pattern.push( [row, var_index] );
                    } else {
                        //
                        // op_index, op_id
                        let op_index  = var_index - var_n_dom;
                        let op_id     = var_id_all[op_index];
                        //
                        if trace {
                            let name = &op_info_vec[op_id as usize].name;
                            println!(
                                "    {} : var_index = {}", name, var_index
                            );
                        }
                        //
                        // var_depend
                        if op_id == CALL_OP || op_id == CALL_RES_OP {
                            cop_depend.clear();
                            dyp_depend.clear();
                            var_depend.clear();
                            call_depend::<V>(
                                &mut atom_depend,
                                &mut cop_depend,
                                &mut dyp_depend,
                                &mut var_depend,
                                &self.var,
                                op_index
                            );
                            for dep_index in var_depend.iter() {
                                var_index_stack.push( *dep_index );
                            }
                            if compute_dyp {
                                for dep_index in dyp_depend.iter() {
                                    dyp_index_stack.push( *dep_index );
                                }
                            }
                        } else {
                            // arg, arg_type
                            let begin    = var_arg_start[op_index] as usize;
                            let end      = var_arg_start[op_index + 1] as usize;
                            let arg      = &var_arg_all[begin .. end];
                            let arg_type = &var_arg_type_all[begin .. end];
                            //
                            // var_index_stack
                            for i in 0 .. arg.len() {
                                if arg_type[i].is_variable() {
                                    var_index_stack.push( arg[i] );
                                } else if arg_type[i].is_dynamic() {
                                    dyp_index_stack.push( arg[i] );
                                }
                            }
                        }
                    }
                }
            }
            // dyp_index
            // range[row] depends on this dynamic parameter
            while ! dyp_index_stack.is_empty() {
                debug_assert!( compute_dyp );
                let dyp_index = dyp_index_stack.pop().unwrap() as usize;
                //
                if dyp_done[dyp_index] != row {
                    dyp_done[dyp_index] = row;
                    if dyp_index < dyp_n_dom {
                        if trace {
                            println!( "    dyp_col = {}", dyp_index );
                        }
                        //
                        // dyp_pattern
                        // dyp_index is a domain variable index
                        dyp_pattern.push( [row, dyp_index] );
                    } else {
                        //
                        // op_index, op_id
                        let op_index  = dyp_index - dyp_n_dom;
                        let op_id     = dyp_id_all[op_index];
                        //
                        if trace {
                            let name = &op_info_vec[op_id as usize].name;
                            println!(
                                "    {} :  dyp_index = {}", name, dyp_index
                        );
                        }
                        //
                        // dyp_index_stack
                        if op_id == CALL_OP || op_id == CALL_RES_OP {
                            cop_depend.clear();
                            dyp_depend.clear();
                            var_depend.clear();
                            call_depend::<V>(
                                &mut atom_depend,
                                &mut cop_depend,
                                &mut dyp_depend,
                                &mut var_depend,
                                &self.dyp,
                                op_index
                            );
                            assert_eq!( var_depend.len(), 0 );
                            for dep_index in dyp_depend.iter() {
                                dyp_index_stack.push( *dep_index );
                            }
                        } else {
                            // arg, arg_type
                            let begin    = dyp_arg_start[op_index] as usize;
                            let end      = dyp_arg_start[op_index + 1] as usize;
                            let arg      = &dyp_arg_all[begin .. end];
                            let arg_type = &dyp_arg_type_all[begin .. end];
                            //
                            // dyp_index_stack
                            for i in 0 .. arg.len() {
                                debug_assert!( ! arg_type[i].is_variable() );
                                if arg_type[i].is_dynamic() {
                                    dyp_index_stack.push( arg[i] );
                                }
                            }
                        }
                    }
                }
            }
        }
        if trace {
            println!( "var_pattern.len() = {}", var_pattern.len() );
            println!( "End Trace: sub_sparsity");
        }
        (dyp_pattern, var_pattern)
    }
}
