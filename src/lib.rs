// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

// YEAR_MONTH_DAY
/// The date corresponding to this version of the software as year.month.day
///
/// # Example
/// ```
/// let version = &*rustad::YEAR_MONTH_DAY;
/// assert_eq!(version, "2025.5.28");
/// ```
pub static YEAR_MONTH_DAY: std::sync::LazyLock<String> =
   std::sync::LazyLock::new( || "2025.5.28".to_string() );


/// Adds two numbers
///
/// # Example
/// ```
#[doc = include_str!("../examples/add.rs")]
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
