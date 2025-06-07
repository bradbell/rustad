// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

// YEAR_MONTH_DAY
/// The date corresponding to this version of the software as year.month.day
///
/// # Example
/// ```
/// let version = &*rustad::YEAR_MONTH_DAY;
/// assert_eq!(version, "2025.6.7");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.7" );

// Index
pub type Index = usize;
//
// Float
pub type Float = f64;
//
// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
    ),
}

// operators
pub const ADD_VC_OP:    Index = 0;
pub const ADD_VV_OP:    Index = ADD_VC_OP + 1;
pub const NUMBER_OP:    Index = ADD_VV_OP + 1;

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
    pub n_independent  : Index,
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
            n_independent : 0,
            n_var         : 0,
            op_vec        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_vec       : Vec::new() ,
            con_vec       : Vec::new() ,
        }
    }
}
//
// THERADS_RECORDER
thread_local! {
    pub static THERADS_RECORDER: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
//
// AD
pub struct AD {
    pub tape_id   : Index,
    pub var_index : Index,
    pub value     : Float,
}
impl From<Float> for AD {
    fn from(this_value : Float) -> Self {
        Self {
            tape_id   : 0,
            var_index : 0,
            value     : this_value,
        }
    }
}
//
// ADFun
pub struct ADFun {
    pub n_independent  : Index,
    pub n_var          : Index,
    pub dependent      : Vec<Index>,
    pub op_vec         : Vec<Index>,
    pub op2arg         : Vec<Index>,
    pub arg_vec        : Vec<Index>,
    pub con_vec        : Vec<Float>,
}
impl ADFun {
    pub fn new() -> Self {
        Self {
            n_independent : 0,
            n_var         : 0,
            op_vec        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_vec       : Vec::new() ,
            con_vec       : Vec::new() ,
            dependent     : Vec::new() ,
        }
    }
}
//
// independent
pub fn independent( x : &[Float] ) {
    THERADS_RECORDER.with_borrow_mut( |tape| {
        assert!( tape.recording );
        assert_eq!( tape.op_vec.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_vec.len(), 0 );
        assert_eq!( tape.con_vec.len(), 0 );
        tape.tape_id       += 1;
        tape.recording      = true;
        tape.n_independent  = x.len();
        tape.n_var          = x.len();
    } );
}
//
// dependent
pub fn dependent( y : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THERADS_RECORDER.with_borrow_mut( |tape| {
        std::mem::swap( &mut result.n_independent, &mut tape.n_independent );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.op_vec,        &mut tape.op_vec );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_vec,       &mut tape.arg_vec );
        std::mem::swap( &mut result.con_vec,       &mut tape.con_vec );
    } );
    for i in 1 .. y.len() {
        result.dependent.push( y[i].var_index );
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
