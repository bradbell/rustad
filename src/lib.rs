// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

// YEAR_MONTH_DAY
/// The date corresponding to this version of the software as year.month.day
///
/// # Example
/// ```
/// let version = &*rustad::YEAR_MONTH_DAY;
/// assert_eq!(version, "2025.6.6");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.6" );

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

// ADD_OP, NUMBER_OP
pub const ADD_OP:    Index = 0;
pub const NUMBER_OP: Index = ADD_OP + 1;

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
    let empty      = OpInfo{ name: "".to_string(), fun : panic_op_fun };
    let mut result = vec![empty ; NUMBER_OP ];
    result[ADD_OP] = OpInfo{ name : "add".to_string() , fun : add_op_fun };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );
//
// TapeInfo
pub struct TapeInfo {
    pub op_vec:     Vec<Index>,
    pub op2arg_vec: Vec<Index>,
    pub arg_vec:    Vec<Index>,
    pub con_vec:    Vec<Float>,
}
impl TapeInfo {
    pub fn new( ) -> Self {
        Self {
            op_vec     : Vec::new() ,
            op2arg_vec : Vec::new() ,
            arg_vec    : Vec::new() ,
            con_vec    : Vec::new() ,
        }
    }
}
//
// THIS_THREADS_TAPE
thread_local! {
    pub static THIS_THREADS_TAPE: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
