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
use crate::numvec::ADfn;
use crate::numvec::tape::Tindex;
use crate::numvec::op::info::GlobalOpInfoVec;
//
#[cfg(doc)]
use crate::numvec::doc_generic_v;
//
// ---------------------------------------------------------------------------
// ADfn::sub_sparsoty
impl<V> ADfn<V>
where
    V : GlobalOpInfoVec,
{
    /// Use the subgraph method to compute a Jacobian sparsity pattern.
    ///
    /// * Syntax :
    /// ```text
    ///     pattern = f.sub_sparsity(trace)
    /// ```
    ///
    /// * V : see [doc_generic_v]
    ///
    /// * f :
    /// is this [ADfn] object. The sparsity pattern is for the Jacobian
    /// of the function defined by the operation sequence stored in f.
    ///
    /// * trace :
    /// If trace is true, a trace of the sparsity calculation
    /// is printed on standard output.
    /// Note that in the trace, the cases where *var_index* is less
    /// that the number of domain variables will end up in the pattern
    /// with the corresponding row.
    ///
    /// * pattern :
    /// The the return value *pattern* is vector of [row, column] pairs.
    /// Each row (column) is less than the range (domain)
    /// dimension for the function.
    /// If a pair [i, j] does not appear, the range component
    /// with index i does not depend on the domain component with index j.
    ///
    /// * dependency :
    /// This is a dependency pattern. For example,
    /// if an range variable was equal to the
    /// Heaviside function of a domain variable,
    /// the corresponding pair would be in the sparisty pattern even though
    /// the corresponding derivative is always zero.
    ///
    /// # Example
    /// ```
    /// use rustad::numvec::AD;
    /// use rustad::numvec::start_recording;
    /// use rustad::numvec::stop_recording;
    /// //
    /// // V
    /// type V = f32;
    /// //
    /// // nx
    /// let nx = 4;
    /// //
    /// let x      : Vec<V>       = vec![2.0; nx];
    /// let ax                    = start_recording(x);
    /// let mut ay : Vec< AD<V> > = Vec::new();
    /// ay.push( AD::from( V::from(5.0) ) ); // ay[0] is a constant
    /// for j in 1 .. nx {
    ///     ay.push( &ax[j] * &ax[j] );      // ay[j] is a variable
    /// }
    /// let f           = stop_recording(ay);
    /// let trace       = false;
    /// let mut pattern = f.sub_sparsity(trace);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), nx - 1 );
    /// for j in 1 .. nx {
    ///     assert_eq!( pattern[j-1], [j,j] );
    /// }
    /// ```
    pub fn sub_sparsity(&self, trace : bool) -> Vec< [usize; 2] >
    {   //
        // zero_tindex
        let zero_tindex  = 0 as Tindex;
        //
        // op_info_vec
        let op_info_vec = &*<V as GlobalOpInfoVec>::get();
        //
        // n_domain ... range2tape_index.
        let n_domain          = self.n_domain;
        let n_var             = self.n_var;
        let flag_all          = &self.flag_all;
        let arg_all           = &self.arg_all;
        let op2arg            = &self.op2arg;
        let range_is_var      = &self.range_is_var;
        let range2tape_index  = &self.range2tape_index;
        //
        // n_range
        let n_range           = range_is_var.len();
        //
        // done
        // initialize all elements as n_var (an invalid variable index)
        let mut done = vec![n_var; n_var];
        //
        // result, arg_var_index, var_index_stack
        let mut result          : Vec< [usize; 2] > = Vec::new();
        let mut arg_var_index   : Vec<Tindex>       = Vec::new();
        let mut var_index_stack : Vec<Tindex>       = Vec::new();
        //
        if trace {
            println!( "n_domain = {}, n_range = {}", n_domain, n_range );
        }
        //
        // row
        // determine the variables that range index row depends on
        for row in 0 .. n_range { if range_is_var[row] {
            //
            // var_index
            let var_index = range2tape_index[row] as usize;
            if trace {
                println!( "row {} var_index {}", row, var_index );
            }
            //
            // var_index_stack
            // use resize instead of new stack to reduce memory allocation
            var_index_stack.resize(0, zero_tindex);
            var_index_stack.push( var_index as Tindex );
            while var_index_stack.len() > 0 {
                //
                // var_index
                let var_index = var_index_stack.pop().unwrap() as usize;
                //
                if done[var_index] != row {
                    done[var_index] = row;
                    if trace {
                        println!( "    var_index = {}", var_index );
                    }
                    if var_index < n_domain {
                        //
                        // result
                        // var_index is a domain variable index
                        result.push( [row, var_index] );
                    } else {
                        //
                        // op_index
                        // the operator that creates this variable
                        let op_index         = var_index - n_domain;
                        //
                        // arv_var_index_fn
                        let op_id            = self.id_all[op_index] as usize;
                        let op_info          = &op_info_vec[op_id];
                        let arg_var_index_fn = op_info.arg_var_index;
                        //
                        // arg
                        let begin = op2arg[op_index] as usize;
                        let end   = op2arg[op_index + 1] as usize;
                        let arg   = &arg_all[begin .. end];
                        //
                        // arg_var_index
                        // the variables that are arguments to this operator
                        arg_var_index_fn(&mut arg_var_index, &flag_all, arg);
                        //
                        // var_index_stack
                        for i in 0 .. arg_var_index.len() {
                            var_index_stack.push( arg_var_index[i] );
                        }
                    }
                }
            }
        } }
        if trace {
            println!( "n_pattern = {}", result.len() );
        }
        result
    }
}
