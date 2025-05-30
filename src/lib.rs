// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : fn(&mut Vec<f64>, usize, usize, usize),
}

// ADD_OP, NUMBER_OP
pub const ADD_OP:    usize = 0;
pub const NUMBER_OP: usize = ADD_OP + 1;

//
// OP_INFO_VEC
fn panic_op_fun(
    _vec: &mut Vec<f64>, _left: usize, _right: usize, _result: usize) {
    panic!();
}
fn add_op_fun(
    vec: &mut Vec<f64>, left: usize, right: usize, result: usize) {
    vec[result] = vec[left] + vec[right];
}
fn op_info_vec() -> Vec<OpInfo> {
    let empty      = OpInfo{ name: "".to_string(), fun : panic_op_fun };
    let mut result = vec![empty ; NUMBER_OP ];
    result[ADD_OP] = OpInfo{ name : "add".to_string() , fun : add_op_fun };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );


// YEAR_MONTH_DAY
/// The date corresponding to this version of the software as year.month.day
///
/// # Example
/// ```
/// let version = &*rustad::YEAR_MONTH_DAY;
/// assert_eq!(version, "2025.5.30");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.5.30" );
