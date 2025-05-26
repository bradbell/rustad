// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

/// The version number as year.month.day
///
/// # Example
/// ```
/// let version = &*rustad::VERSION;
///
/// assert_eq!(version, "2025.5.26");
/// ```
pub static VERSION: std::sync::LazyLock<String> =
   std::sync::LazyLock::new( || "2025.5.26".to_string() );


/// Adds two numbers
///
/// # Example
///
/// ```
/// let left  = 5;
/// let right = 6;
/// let answer = rustad::add(left, right);
///
/// assert_eq!(11, answer);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test;
