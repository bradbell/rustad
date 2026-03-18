// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Implements [ADfn] forward sparsity method.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
//
use crate::tape::AGraph;
use crate::vec_set::VecSet;
use crate::op::info::OpFns;
use crate::op::call::call_depend;
use crate::atom::AtomCallback;
use crate::{
    ADfn,
    IndexT,
    GlobalAtomCallbackVecPublic,
    SparsityPattern,
};
use crate::op::id::{
    CALL_OP,
    CALL_RES_OP,
};
use crate::op::info::{
    sealed::GlobalOpFnsVec,
};
//
#[cfg(doc)]
use crate::doc_generic_v;
// ----------------------------------------------------------------------------
// ADfn::for_sparsity
impl<V> ADfn<V>
where
    V               : GlobalAtomCallbackVecPublic + GlobalOpFnsVec ,
    AtomCallback<V> : Clone,
{
    /// Use the forward mode to compute a Jacobian sparsity pattern.
    ///
    /// See Also : [ADfn::sub_sparsity]
    ///
    /// * Syntax :
    ///   ```text
    ///     pattern = f.for_sparsity(opt_vec)
    ///   ```
    ///
    /// * V : see [doc_generic_v]
    ///
    /// * f :
    ///   is this [ADfn] object. The sparsity pattern is for the Jacobian
    ///   of the function defined by the acyclic graphs stored in f.
    ///
    /// * opt_vec :
    ///   is an [opt_vec](crate::doc_opt_vec) with the following possible keys:
    ///
    ///   * trace
    ///     The corresponding value must be true of false (default is false).
    ///     If it is true, a trace of for_sparsity is printed on stdout.
    ///
    ///   * compute_dyp :
    ///     The corresponding value must be true of false (default is false).
    ///     If this is true, the return is a sparsity pattern
    ///     for the range of f w.r.t. the domain dynamic parameters.
    ///     Otherwise, the sparsity pattern is w.r.t. the domain variables.
    ///     The default for compute_dyp is false.
    ///
    /// * pattern :
    ///   The the return value *pattern* is vector of [row, column] pairs.
    ///   Each row is a range index and is less that [ADfn::rng_len] .
    ///   If compute_dyp is true (false) eah column is a
    ///   dynamic parameter (variable) domain index and is less than
    ///   [ADfn::dyp_dom_len] ( [ADfn::var_dom_len] ).
    ///   If a pair [i, j] does not appear, the range component
    ///   with index i does not depend on the domain component with index j.
    ///
    /// * dependency :
    ///   This is a dependency pattern. For example,
    ///   if an range variable was equal to the
    ///   Heaviside function of a domain variable,
    ///   the corresponding pair would be in the sparisty pattern even though
    ///   the corresponding derivative is always zero.
    ///
    /// # Example
    /// ```
    /// use rustad::AD;
    /// use rustad::start_recording;
    /// use rustad::stop_recording;
    /// //
    /// // V
    /// type V = rustad::AzFloat<f32>;
    /// //
    /// // nx
    /// let nx = 4;
    /// //
    /// // opt_vec
    /// let opt_vec : Vec<[&str; 2]> = Vec::new();
    /// //
    /// let x      : Vec<V>       = vec![ V::from(2.0); nx];
    /// let (_, ax)               = start_recording(None, x);
    /// let mut ay : Vec< AD<V> > = Vec::new();
    /// ay.push( AD::from( V::from(5.0) ) ); // ay[0] is a constant
    /// for j in 1 .. nx {
    ///     ay.push( &ax[j] * &ax[j] );      // ay[j] is a variable
    /// }
    /// let f           = stop_recording(ay);
    /// let mut pattern = f.for_sparsity(&opt_vec);
    /// pattern.sort();
    /// assert_eq!( pattern.len(), nx - 1 );
    /// for j in 1 .. nx {
    ///     assert_eq!( pattern[j-1], [j,j] );
    /// }
    /// ```
    ///
    pub fn for_sparsity(
        &self                      ,
        opt_vec : &Vec<[&str; 2]>  ,
    ) -> SparsityPattern
    {   //
        // trace, compute_dyp
        let mut trace       = false;
        let mut compute_dyp = false;
        for opt in opt_vec {
            match opt[0] {
                "trace" => {
                    match opt[1] {
                        "true"  => { trace = true; },
                        "false" => { trace = false; },
                        _ => { panic!(
                        "for_sparsity opt_vec: invalid value for trace"
                        ); }
                    }
                },
                "compute_dyp" => {
                    match opt[1] {
                        "true"  => { compute_dyp = true; },
                        "false" => { compute_dyp = false; },
                        _ => { panic!(
                        "for_sparsity opt_vec: invalid value for compute_dyp"
                        ); }
                    }
                },
                _ => panic!("for_sparsity opt_vec: invalid key"),
            }
        }
        //
        // op_fns_vec
        let op_fns_vec : &Vec< OpFns<V> >  = GlobalOpFnsVec::get();
        //
        // rng_ad_type, range_ad_index, n_range
        let rng_ad_type       = &self.rng_ad_type;
        let rng_index         = &self.rng_index;
        let n_range           = rng_ad_type.len();
        //
        // pattern, depend_usize
        let mut pattern         : SparsityPattern   = Vec::new();
        let mut depend_usize    : Vec<usize>        = Vec::new();
        //
        // atom_depend, cop_depend, dyp_depend, var_depend
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut cop_depend  : Vec<IndexT> = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // n_agraph, n_dyp, set_vec
        let n_agraph     : usize;
        let n_dyp        : usize;
        let mut set_vec  : VecSet = VecSet::new();
        if compute_dyp {
            n_agraph  = 2;
            n_dyp     = self.dyp.n_dom + self.dyp.n_dep;
            for id_set in 0 .. self.dyp.n_dom {
                set_vec.singleton( id_set );
            }
        } else {
            n_agraph  = 1;
            n_dyp     = 0;
            for id_set in 0 .. self.var.n_dom {
                set_vec.singleton( id_set );
            }
        }
        //
        if trace {
            let mut range_set_index : Vec<usize> = Vec::new();
            for i in 0 .. rng_index.len() {
                if rng_ad_type[i].is_variable() {
                        let index = (rng_index[i] as usize) + n_dyp;
                        range_set_index.push( index );
                }
                if rng_ad_type[i].is_dynamic() && compute_dyp {
                        range_set_index.push(  rng_index[i] as usize );
                }
            }
            let n_dom =
                if compute_dyp { self.dyp.n_dom } else { self.var.n_dom};
            println!( "Begin Trace: for_sparisty:" );
            println!( "compute_dyp = {}, n_dom = {}" , compute_dyp, n_dom);
            println!("range_set_index = {:?}", range_set_index);
            println!("var_index, op_name, var_arguments, set_result");
        }
        // i_agraph
        for i_agraph in 0 .. n_agraph {
            //
            // agraph
            let agraph : &AGraph;
            if i_agraph == 1 {
                debug_assert!( compute_dyp );
                agraph = &self.var;
                for j in 0 .. self.var.n_dom {
                    // set_vec
                    // domain variables don't depend on dynamic parameters
                    let id_set = set_vec.empty();
                    assert_eq!(id_set, j + n_dyp);
                }
            } else if compute_dyp {
                agraph = &self.dyp;
            } else {
                agraph = &self.var;
            }
            //
            // n_dom, n_dep, id_all, arg_start, arg_all, atr_type_all
            let n_dom             = agraph.n_dom;
            let n_dep             = agraph.n_dep;
            let id_all            = &agraph.id_all;
            let arg_start         = &agraph.arg_start;
            let arg_all           = &agraph.arg_all;
            let arg_type_all      = &agraph.arg_type_all;
            //
            // op_index
            for op_index in 0 .. n_dep {
                //
                // op_id
                let op_id = id_all[op_index];
                //
                // depend_usize
                depend_usize.clear();
                if op_id == CALL_OP || op_id == CALL_RES_OP {
                    cop_depend.clear();
                    dyp_depend.clear();
                    var_depend.clear();
                    call_depend::<V>(
                        &mut atom_depend,
                        &mut cop_depend,
                        &mut dyp_depend,
                        &mut var_depend,
                        agraph,
                        op_index
                    );
                    for dep_index in var_depend.iter() {
                        debug_assert!( i_agraph != 0 || ! compute_dyp );
                        depend_usize.push(*dep_index as usize + n_dyp );
                    }
                    if compute_dyp {
                        for dep_index in dyp_depend.iter() {
                            depend_usize.push( *dep_index as usize );
                        }
                    }
                } else {
                    //
                    // arg, arg_type
                    let begin      = arg_start[op_index] as usize;
                    let end        = arg_start[op_index + 1] as usize;
                    let arg        = &arg_all[begin .. end];
                    let arg_type   = &arg_type_all[begin .. end];
                    //
                    // depend_usize
                    for i in 0 .. arg.len() {
                        if arg_type[i].is_variable() {
                            debug_assert!( i_agraph != 0 || ! compute_dyp );
                            depend_usize.push(  arg[i] as usize + n_dyp );
                        }
                        if compute_dyp && arg_type[i].is_dynamic() {
                            depend_usize.push(  arg[i] as usize );
                        }
                    }
                }
                //
                // dep_index
                let dep_index = if compute_dyp && i_agraph == 0 {
                    n_dom + op_index
                } else {
                    n_dom + op_index + n_dyp
                };
                //
                // set_vec.get(set_id)
                let set_id = set_vec.union( &depend_usize );
                assert_eq!(dep_index,  set_id);
                //
                if trace {
                    let op_id   = id_all[op_index] as usize;
                    let op_name = &op_fns_vec[op_id].name;
                    let set     = set_vec.get(dep_index);
                    println!(
                        "{}, {}, {:?}, {:?}",
                        dep_index, op_name, depend_usize,  set
                    );
                }
            }
        }
        for i in 0 .. n_range {
            if rng_ad_type[i].is_variable() {
                let row_var_index = rng_index[i] as usize + n_dyp;
                let set           = set_vec.get(row_var_index);
                for j in set {
                    pattern.push( [i, *j] );
                }
            }
            if compute_dyp && rng_ad_type[i].is_dynamic() {
                let row_var_index = rng_index[i] as usize;
                let set           = set_vec.get(row_var_index);
                for j in set {
                    pattern.push( [i, *j] );
                }
            }
        }
        if trace {
            println!( "n_pattern = {}", pattern.len() );
            println!( "End Trace: for_sparisty:" );
        }
        pattern
    }
}
