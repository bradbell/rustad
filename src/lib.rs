// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.6.10");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.10" );
// ----------------------------------------------------------------------------
//
// utility
pub mod utility;
//
// ADD_VV_OP, ADD_VC_OP, ...
// define all the operator indices
pub(crate) mod op_index;
use op_index::*;
// ----------------------------------------------------------------------------
// Index
/// Type used for indexing vectors in the tape.
/// It must be able to represent the total number of
/// operators, constants, and arguments to operators.
pub type Index = usize;
//
// Float
/// Floating point Type used for AD operations.
pub type Float = f64;
//
// ForwardZeroFn
/// Type used for fuunctions that evaluate zero order forward mode
pub type ForwardZeroFn = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
// ----------------------------------------------------------------------------
//
// AD
pub mod ad;
use ad::AD;
//
// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : ForwardZeroFn,
}

//
// panic_eval_fn
fn panic_eval_fn(
    _vec: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// TapeInfo
pub struct TapeInfo {
    pub tape_id        : Index,
    pub recording      : bool,
    pub n_domain       : Index,
    pub n_var          : Index,
    pub op_vec         : Vec<Index>,
    pub op2arg         : Vec<Index>,
    pub arg_vec        : Vec<Index>,
    pub con_vec        : Vec<Float>,
}
impl TapeInfo {
    pub fn new() -> Self {
        Self {
            tape_id       : 0,
            recording     : false,
            n_domain      : 0,
            n_var         : 0,
            op_vec        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_vec       : Vec::new() ,
            con_vec       : Vec::new() ,
        }
    }
}
//
// THIS_THREAD_TAPE
thread_local! {
    pub static THIS_THREAD_TAPE: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
//
// ADFun
pub struct ADFun {
    pub n_domain       : Index,
    pub n_var          : Index,
    pub range          : Vec<Index>,
    pub op_vec         : Vec<Index>,
    pub op2arg         : Vec<Index>,
    pub arg_vec        : Vec<Index>,
    pub con_vec        : Vec<Float>,
}
impl ADFun {
    pub fn new() -> Self {
        Self {
            n_domain      : 0,
            n_var         : 0,
            op_vec        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_vec       : Vec::new() ,
            con_vec       : Vec::new() ,
            range         : Vec::new() ,
        }
    }
    pub fn forward(&self, x : &[Float] ) -> Vec<Float> {
        let op_info_vec = &*OP_INFO_VEC;
        let mut var_vec = vec![ Float::NAN; self.n_var ];
        for j in 0 .. self.n_domain {
            var_vec[j] = x[j];
        }
        for i_op in 0 .. self.op_vec.len() {
            let op    = self.op_vec[i_op];
            let start = self.op2arg[i_op];
            let end   = self.op2arg[i_op + 1];
            let arg   = &self.arg_vec[start .. end];
            let res   = self.n_domain + i_op;
            let fun   = op_info_vec[op].fun;
            fun(&mut var_vec, &self.con_vec, &arg, res );
        }
        let mut y : Vec<Float> = Vec::new();
        for i in 0 .. self.range.len() {
            y.push( var_vec[ self.range[i] ] );
        }
        y
    }
}
//
// domain
pub fn domain( x : &[Float] ) -> Vec<AD> {
    let mut new_tape_id = 0;
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.op_vec.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_vec.len(), 0 );
        assert_eq!( tape.con_vec.len(), 0 );
        tape.tape_id       += 1;
        tape.recording      = true;
        tape.n_domain       = x.len();
        tape.n_var          = x.len();
        //
        new_tape_id         = tape.tape_id;
    } );
    let mut result : Vec<AD> = Vec::new();
    for j in 0 .. x.len() {
        result.push(
             AD { tape_id : new_tape_id, var_index : j, value : x[j] }
        );
    }
    result
}
//
// range
pub fn range( y : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        tape.op2arg.push( tape.arg_vec.len() );
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        std::mem::swap( &mut result.n_domain, &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.op_vec,        &mut tape.op_vec );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_vec,       &mut tape.arg_vec );
        std::mem::swap( &mut result.con_vec,       &mut tape.con_vec );
    } );
    for i in 0 .. y.len() {
        result.range.push( y[i].var_index );
    }
    result
}
//
// add_op
mod add_op;
//
// OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), fun : panic_eval_fn };
    let mut result    = vec![empty ; NUMBER_OP ];
    result[ADD_VC_OP] =
        OpInfo{ name : "add_vc".to_string() , fun : add_op::eval_add_vc_fn };
    result[ADD_VV_OP] =
        OpInfo{ name : "add_vv".to_string() , fun : add_op::eval_add_vv_fn };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );

#[test]
fn test_op_info() {
    let op_info_vec = &*OP_INFO_VEC;
    assert_eq!( "add_vc", op_info_vec[ADD_VC_OP].name );
    assert_eq!( "add_vv", op_info_vec[ADD_VV_OP].name );
}
