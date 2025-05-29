// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
}
pub const ADD_OP:    usize = 0;
pub const NUMBER_OP: usize = ADD_OP + 1;

fn op_info_vec() -> Vec<OpInfo> {
    let mut result = vec![ OpInfo{ name : "".to_string() } ; NUMBER_OP ];
    result[ADD_OP] = OpInfo{ name : "add".to_string() };
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
/// assert_eq!(version, "2025.5.29");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.5.29" );


/// Adds two numbers
///
/// # Example
/// ```
#[doc = include_str!("../examples/add.rs")]
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
