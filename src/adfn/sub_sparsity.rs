// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Implements [ADfn] sub-graph sparsity method.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
//
use crate::ADfn;
use crate::IndexT;
use crate::op::call::call_depend;
use crate::atom::sealed::AtomInfoVec;
use crate::atom::AtomCallback;
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
    V               : AtomInfoVec,
    AtomCallback<V> : Clone,
{
    /// Use the subgraph method to compute a Jacobian sparsity pattern.
    ///
    /// * See Also : [ADfn::for_sparsity]
    ///
    /// * Syntax :
    /// ```text
    ///     (var_pattern, dyp_pattern) = f.sub_sparsity(trace, compute_dyp)
    /// ```
    ///
    /// * V : see [doc_generic_v]
    ///
    /// * f :
    /// is this [ADfn] object. The sparsity pattern is for the Jacobian
    /// of the function defined by the operation sequence stored in f.
    ///
    /// * trace :
    /// If this is true, a trace of the sparsity calculation
    /// is printed on standard output.
    /// Note that in the trace, the cases where *var_index* is less
    /// that the number of domain variables will end up in the pattern
    /// with the corresponding row.
    ///
    /// * compute_dyp :
    /// If this is true (false),
    /// the dynamic parameter pattern is (is not) computed.
    ///
    /// * var_pattern :
    /// This return is vector of [row, column] pairs.
    /// Each row (column) is less than the range (variable domain)
    /// dimension for this function.
    /// If a pair [i, j] does not appear, the range component
    /// with index i does not depend on the domain variable with index j.
    ///
    /// * dyp_pattern (Under Construction) :
    /// This return is vector of [row, column] pairs.
    /// Each row (column) is less than the range (dynamic parameter domain)
    /// dimension for this function.
    /// If a pair [i, j] does not appear, the range component
    /// with index i does not depend on the domain dynamic parameter with index j.
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
    /// use rustad::start_recording_var;
    /// use rustad::stop_recording;
    /// //
    /// // V
    /// type V = rustad::AzFloat<f32>;
    /// //
    /// // nx, x, ax
    /// let nx = 4;
    /// let x                     = vec![ V::from(2.0); nx];
    /// let ax                    = start_recording_var(x);
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
    /// let (mut pattern, _) = f.sub_sparsity(trace, compute_dyp);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), nx - 1 );
    /// for j in 1 .. nx {
    ///     assert_eq!( pattern[j-1], [j, j] );
    /// }
    /// ```
    pub fn sub_sparsity(
        &self, trace : bool, _compute_dyp : bool
    ) -> ( Vec< [usize; 2] > , Vec< [ usize; 2 ] > )
    {   //
        // n_dom ... range_index.
        let n_dom             = self.var.n_dom;
        let n_dep             = self.var.n_dep;
        let id_seq            = &self.var.id_seq;
        let arg_seq           = &self.var.arg_seq;
        let arg_all           = &self.var.arg_all;
        let arg_type_all      = &self.var.arg_type_all;
        let range_ad_type     = &self.range_ad_type;
        let range_index       = &self.range_index;
        //
        // n_range
        let n_range           = range_ad_type.len();
        //
        // done
        // initialize all elements as n_var (an invalid variable index)
        let n_var    = n_dom + n_dep;
        let mut done = vec![n_var; n_var];
        //
        // result, var_index_stack
        let mut result          : Vec< [usize; 2] > = Vec::new();
        let mut var_index_stack : Vec<IndexT>       = Vec::new();
        //
        if trace { println!(
            "Begin Trace: sub_sparsity: n_dom = {}, n_range = {}",
            n_dom, n_range
        ); }
        //
        // atom_depend, dyp_depend, var_depend
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // row
        // determine the variables that range index row depends on
        for row in 0 .. n_range { if range_ad_type[row].is_variable() {
            //
            // var_index
            let var_index = range_index[row] as usize;
            if trace {
                println!( "row = {}", row );
            }
            //
            // var_index_stack
            // use resize instead of new stack to reduce memory allocation
            var_index_stack.clear();
            var_index_stack.push( var_index as IndexT );
            while var_index_stack.len() > 0 {
                //
                // var_index
                let var_index = var_index_stack.pop().unwrap() as usize;
                //
                if done[var_index] != row {
                    done[var_index] = row;
                    if var_index < n_dom {
                        if trace {
                            println!( "    col = {}", var_index );
                        }
                        //
                        // result
                        // var_index is a domain variable index
                        result.push( [row, var_index] );
                    } else {
                        if trace {
                            println!( "    var_index = {}", var_index );
                        }
                        //
                        // op_index, op_id
                        let op_index  = var_index - n_dom;
                        let op_id     = id_seq[op_index];
                        //
                        // var_depend
                        if op_id == CALL_OP || op_id == CALL_RES_OP {
                            dyp_depend.clear();
                            var_depend.clear();
                            call_depend::<V>(
                                &mut atom_depend,
                                &mut dyp_depend,
                                &mut var_depend,
                                &self.var,
                                op_index
                            );
                            for dep_index in var_depend.iter() {
                                var_index_stack.push( dep_index.clone() );
                            }
                        } else {
                            // arg, arg_type
                            let begin    = arg_seq[op_index] as usize;
                            let end      = arg_seq[op_index + 1] as usize;
                            let arg      = &arg_all[begin .. end];
                            let arg_type = &arg_type_all[begin .. end];
                            //
                            // var_index_stack
                            for i in 0 .. arg.len() {
                                if arg_type[i].is_variable() {
                                    var_index_stack.push( arg[i] );
                                }
                            }
                        }
                    }
                }
            }
        } }
        if trace {
            println!( "n_pattern = {}", result.len() );
        }
        let empty : Vec< [usize; 2] > = Vec::new();
        (result, empty)
    }
}
