// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! ADFun objects
//
use crate::Index;
use crate::Float;
use crate::operator::OP_INFO_VEC;
use crate::AD;
use crate::ad_tape::THIS_THREAD_TAPE;
use crate::ad_tape::NEXT_TAPE_ID;
//
#[cfg(doc)]
use crate::operator;
//
// ADFun
/// This object can evaluate an operation sequence amd its derivatives.
///
/// # Operation sequence
/// An operation sequence is a single assignment representation of
/// the function; i.e., a variable is only assigned once.
///
/// # Constructor
/// An [ad_domain] call is used to start a recording an operation sequence.
/// An [ad_fun] call is used to stop recording move the operation sequence
/// to an new ADFun object.
pub struct ADFun {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain       : Index,
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub(crate) n_var          : Index,
    //
    // range_index
    /// The variable index for each of the range variables in this function.
    /// The dimension of its range spase is range_index.len().
    pub(crate) range_index    : Vec<Index>,
    //
    // id_all
    /// This maps an operator's index in the operation sequence
    /// to its [operator::id]
    pub(crate) id_all         : Vec<Index>,
    //
    // op2arg
    /// This maps an operator's index in the operation sequence to its
    /// the index of its first argument in arg_all.
    pub(crate) op2arg         : Vec<Index>,
    //
    // arg_all
    /// This contains the arguments for all the opereators in the
    /// operatioon sequence.
    pub(crate) arg_all        : Vec<Index>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_all        : Vec<Float>,
}
// ---------------------------------------------------------------------------
impl ADFun {
    //
    // new
    /// This creates an empty operation sequence; i.e,
    /// its domain and range vectors have length zero.
    /// # Example
    /// ```
    /// let f = rustad::function::ADFun::new();
    /// assert_eq!( f.len_domain(), 0 );
    /// assert_eq!( f.len_range(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain      : 0,
            n_var         : 0,
            id_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
            range_index   : Vec::new() ,
        }
    }
    //
    // len_domain
    /// dimension of domain space
    pub fn len_domain(&self) -> Index { self.n_domain }
    //
    // len_range
    /// dimension of range space
    pub fn len_range(&self) -> Index { self.range_index.len() }
    //
    // -----------------------------------------------------------------------
    // forward_zero
    /// zero order forward mode function evaluation.
    ///
    /// # Syntax
    /// <pre>
    ///     (range_zero, var_zero) = f.forward(domain_zero, trace)
    /// </pre>
    ///
    /// # f
    /// is is this ADFun object.
    ///
    /// # domain_zero
    /// specifies the domain space variable values.
    ///
    /// # trace
    /// if true, a trace of the operatiopn sequence is printed on stdout.
    ///
    /// # range_zero
    /// The first return value is the range vector corresponding to
    /// domain_zero;
    /// i.e., the function value correspdong the operation sequence.
    ///
    /// # var_zero
    /// The second return value is the value for all the variables
    /// in the operation sequence. This is needed to compute derivatives.
    pub fn forward_zero(
        &self,
        domain_zero : &[Float],
        trace       : bool
    ) -> ( Vec<Float> , Vec<Float> ) {
        assert_eq!(
            domain_zero.len(), self.n_domain,
            "f.forward_zero: domain_zero length does not match f"
        );
        //
        let op_info_vec = &*OP_INFO_VEC;
        let mut var_zero = vec![ Float::NAN; self.n_var ];
        for j in 0 .. self.n_domain {
            var_zero[j] = domain_zero[j];
        }
        if trace {
            println!( "index, constant" );
            for j in 0 .. self.con_all.len() {
                println!( "{:?}, {:?}", j, self.con_all[j] );
            }
            println!( "index, domain_zero" );
            for j in 0 .. domain_zero.len() {
                println!( "{:?}, {:?}", j, var_zero[j] );
            }
            println!( "res. name, arg,. var_zero" );
        }
        for op_index in 0 .. self.id_all.len() {
            let op_id     = self.id_all[op_index];
            let start     = self.op2arg[op_index];
            let end       = self.op2arg[op_index + 1];
            let arg       = &self.arg_all[start .. end];
            let res       = self.n_domain + op_index;
            let forward_0 = op_info_vec[op_id].forward_0;
            forward_0(&mut var_zero, &self.con_all, &arg, res );
            if trace {
                let name = &op_info_vec[op_id].name;
                println!(
                    "{:?}, {:?}, {:?}, {:?}", res, name, arg, var_zero[res]
                );
            }
        }
        let mut range_zero : Vec<Float> = Vec::new();
        for i in 0 .. self.range_index.len() {
            range_zero.push( var_zero[ self.range_index[i] ] );
        }
        ( range_zero, var_zero )
    }
    // -----------------------------------------------------------------------
    // forward_one
    /// first order forward mode function evaluation.
    ///
    /// # Syntax
    /// <pre>
    ///     range_one = f.forward(domain_one, var_zero, trace)
    /// </pre>
    ///
    /// # f
    /// is is this ADFun object.
    ///
    /// # domain_one
    /// specifies the directional deriva=tive for domain space variables.
    ///
    /// # var_zero
    /// is the value for all the variables in the operation sequence.
    /// This is returned at the end of a [forward_zero](ADFun::forward_zero)
    /// computation.
    ///
    /// # trace
    /// if true, a trace of the operatiopn sequence is printed on stdout.
    ///
    /// # range_one
    /// The return value is the range vector corresponding to
    /// domain_one and var_zero;
    /// i.e., the directional derivative for the fuctioon
    /// corresponding to the operation sequence.
    pub fn forward_one(
        &self,
        domain_one : &[Float],
        var_zero   : &Vec<Float>,
        trace      : bool
    ) -> Vec<Float> {
        assert_eq!(
            domain_one.len(), self.n_domain,
            "f.forward_one: domain_one length does not match f"
        );
        assert_eq!(
            var_zero.len(), self.n_var,
            "f.forward_one: var_zero length does not match f"
         );
        //
        let op_info_vec = &*OP_INFO_VEC;
        let mut var_one = vec![ Float::NAN; self.n_var ];
        for j in 0 .. self.n_domain {
            var_one[j] = domain_one[j];
        }
        if trace {
            println!( "index, constant" );
            for j in 0 .. self.con_all.len() {
                println!( "{:?}, {:?}", j, self.con_all[j] );
            }
            println!( "index, domain_zero, domain_one" );
            for j in 0 .. domain_one.len() {
                println!( "{:?}, [{:?}, {:?}]", j, var_zero[j], var_one[j] );
            }
            println!( "res, name, arg, var_zero[res]. var_one[res]" )
        }
        for op_index in 0 .. self.id_all.len() {
            let op_id     = self.id_all[op_index];
            let start     = self.op2arg[op_index];
            let end       = self.op2arg[op_index + 1];
            let arg       = &self.arg_all[start .. end];
            let res       = self.n_domain + op_index;
            let forward_1 = op_info_vec[op_id].forward_1;
            forward_1(&mut var_one, var_zero, &self.con_all, &arg, res );
            if trace {
                let name = &op_info_vec[op_id].name;
                println!(
                    "{:?}, {:?}, {:?}, [{:?}, {:?}]",
                    res, name, arg, var_zero[res], var_one[res]
                );
            }
        }
        let mut range_one : Vec<Float> = Vec::new();
        for i in 0 .. self.range_index.len() {
            range_one.push( var_one[ self.range_index[i] ] );
        }
        range_one
    }
    // -------------------------------------------------------------------
    // reverse_one
    /// first order reverse mode evaluation of partial dervatives.
    ///
    /// # Syntax
    /// <pre>
    ///     domain_one = f.reverse_one(range_one, var_zero, trace)
    /// </pre>
    ///
    /// # f
    /// is is this ADFun object.
    ///
    /// # ramge_one
    /// specifies the partials of as scalar function of range variables.
    ///
    /// # var_zero
    /// is the value for all the variables in the operation sequence.
    /// This is returned at the end of a [forward_zero](ADFun::forward_zero)
    /// computation.
    ///
    /// # trace
    /// if true, a trace of the operatiopn sequence is printed on stdout.
    ///
    /// # domain_one
    /// The return value is the partials of the scalar function
    /// with respect to the domain variables.
    pub fn reverse_one(
        &self,
        range_one  : &[Float],
        var_zero   : &Vec<Float>,
        trace      : bool
    ) -> Vec<Float> {
        assert_eq!(
            range_one.len(), self.range_index.len(),
            "f.reverse_one: range_one length does not match f"
        );
        assert_eq!(
            var_zero.len(), self.n_var,
            "f.reverse_one: var_zero length does not match f"
         );
        //
        let op_info_vec = &*OP_INFO_VEC;
        let mut partial = vec![0.0; self.n_var ];
        for j in 0 .. self.range_index.len() {
            partial[ self.range_index[j] ] += range_one[j];
        }
        if trace {
            println!( "index, constant" );
            for j in 0 .. self.con_all.len() {
                println!( "{:?}, {:?}", j, self.con_all[j] );
            }
            println!( "index, range_zero, range_one" );
            for j in 0 .. range_one.len() {
                println!( "{:?}, [{:?}, {:?}]", j, var_zero[j], partial[j] );
            }
            println!( "res, name, arg, var_zero[res]. partial[res]" )
        }
        for op_index in ( 0 .. self.id_all.len() ).rev() {
            let op_id     = self.id_all[op_index];
            let start     = self.op2arg[op_index];
            let end       = self.op2arg[op_index + 1];
            let arg       = &self.arg_all[start .. end];
            let res       = self.n_domain + op_index;
            let reverse_1 = op_info_vec[op_id].reverse_1;
            reverse_1(&mut partial, var_zero, &self.con_all, &arg, res );
            if trace {
                let name = &op_info_vec[op_id].name;
                println!(
                    "{:?}, {:?}, {:?}, [{:?}, {:?}]",
                    res, name, arg, var_zero[res], partial[res]
                );
            }
        }
        let mut domain_one : Vec<Float> = Vec::new();
        for j in 0 .. self.n_domain {
            domain_one.push( partial[j] );
        }
        domain_one
    }
}
//
// ad_domain
/// Calling `ad_domain` function starts a new recording.
///
/// # Recording
/// There must not currently be a recording in process on the current thread.
///
/// # domain
/// This vector determines the number of domain (independent) variables
/// and their value during the recording.
///
/// # ad_domain
/// The return value is the vector of domain space variables.
/// It has the same length and values as domain.
pub fn ad_domain( domain : &[Float] ) -> Vec<AD> {
    //
    // new_tape_id
    let new_tape_id : Index;
    {   let mut next_tape_id = NEXT_TAPE_ID.lock().unwrap();
        //
        // The rest pf this block has a lock, so it is fast and can't fail.
        new_tape_id   = *next_tape_id;
        *next_tape_id = new_tape_id + 1;
    }
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        assert_ne!( new_tape_id, 0);
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.id_all.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_all.len(), 0 );
        assert_eq!( tape.con_all.len(), 0 );
        tape.tape_id        = new_tape_id;
        tape.recording      = true;
        tape.n_domain       = domain.len();
        tape.n_var          = domain.len();
        //
    } );
    let mut result : Vec<AD> = Vec::new();
    for j in 0 .. domain.len() {
        result.push(
             AD { tape_id : new_tape_id, var_index : j, value : domain[j] }
        );
    }
    result
}
//
// ad_fun
/// Calling `ad_fun` stops a recordng and moves it to an ADFun object..
///
/// # Recording
/// There must currently be a recording in process on the current thread.
///
/// # ad_range
/// This is an AD vector of range space variables.
///
/// # ad_fun
/// The return value is an ADFun containing the sequence of operations
/// that compute the range space variables as a function of the
/// domain space variables.
pub fn ad_fun( ad_range : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        tape.op2arg.push( tape.arg_all.len() );
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        std::mem::swap( &mut result.n_domain, &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.id_all,        &mut tape.id_all );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut result.con_all,       &mut tape.con_all );
    } );
    for i in 0 .. ad_range.len() {
        result.range_index.push( ad_range[i].var_index );
    }
    result
}
