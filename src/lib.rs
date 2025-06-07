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

// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : fn(&mut Vec<Float>, &[Index], Index),
}

// operators
pub const ADD_VC_OP:    Index = 0;
pub const ADD_VV_OP:    Index = ADD_VC_OP + 1;
pub const NUMBER_OP:    Index = ADD_VV_OP + 1;

//
// OP_INFO_VEC
fn panic_op_fun(
    _vec: &mut Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
fn add_op_fun(
    vec: &mut Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    vec[ res ] = vec[ arg[0] ] + vec[ arg[1] ];
}
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), fun : panic_op_fun };
    let mut result    = vec![empty ; NUMBER_OP ];
    result[ADD_VV_OP] = OpInfo{ name : "add".to_string() , fun : add_op_fun };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );
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
// THIS_THREADS_TAPE
thread_local! {
    pub static THIS_THREADS_TAPE: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
//
// AD
struct AD {
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
// std::ops::ADD for AD
fn record_add(tape : &mut TapeInfo, lhs : &AD, rhs : &AD) -> (Index, Index) {
    let mut new_tape_id   = 0;
    let mut new_var_index = 0;
    if tape.recording {
        let var_lhs    = lhs.tape_id == tape.tape_id;
        let var_rhs    = rhs.tape_id == tape.tape_id;
        if var_lhs || var_rhs {
            new_tape_id = tape.tape_id;
            if var_lhs && var_rhs {
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op_vec.push(ADD_VV_OP);
                tape.op2arg.push( tape.arg_vec.len() );
                tape.arg_vec.push( lhs.var_index );
                tape.arg_vec.push( rhs.var_index );
            } else if var_lhs {
                if rhs.value == 0.0 {
                    new_var_index = lhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.op_vec.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_vec.len() );
                    tape.arg_vec.push( lhs.var_index );
                    tape.arg_vec.push( tape.con_vec.len() );
                    tape.con_vec.push( rhs.value );
                }
            } else {
                if lhs.value == 0.0 {
                    new_var_index = rhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.op_vec.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_vec.len() );
                    tape.arg_vec.push( rhs.var_index );
                    tape.arg_vec.push( tape.con_vec.len() );
                    tape.con_vec.push( lhs.value );
                }
            }
        }
    }
    (new_tape_id, new_var_index)
}
impl std::ops::Add for AD {
    type Output = AD;
    fn add(self, rhs : AD) -> AD
    {   let new_value                     = self.value + rhs.value;
        let ( new_tape_id, new_var_index) = THIS_THREADS_TAPE.with_borrow_mut(
            |tape| record_add(tape, &self, &rhs)
        );
        AD {
            tape_id   : new_tape_id,
            var_index : new_var_index,
            value     : new_value,
        }
    }
}
//
// independent
pub fn independent( x : &[Float] ) {
    THIS_THREADS_TAPE.with_borrow_mut( |tape| {
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
