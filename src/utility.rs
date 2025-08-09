// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Some utilities not specific to implementing the rustd package
//!
//! Link to [parent module](super)
//
// avg_seconds_to_execute
/// Compute the average time required to execute a function.
///
/// * fun :
/// The function that we are executing.
///
/// * total_seconds :
/// The execution will be repeated until the total execution time is at least
/// *total_seconds* .
///
/// # return :
/// The return value is the total execution time, in seconds,
/// divided by the number of repeats; i.e, the average per call to *fun* .
///
/// # Example
/// ```
/// use rustad::utility::avg_seconds_to_execute;
/// let faster = || { println!( "faster: ");
/// };
/// let slower = || {
///     let mut sum  = 0;
///     for i in 0 .. 1000 { sum += i; }
///     println!( "slower: {},", sum);
/// };
/// let total_seconds = 0.5;
/// let s1 = avg_seconds_to_execute(faster, total_seconds);
/// let s2 = avg_seconds_to_execute(slower, total_seconds);
/// assert!( s1 < s2 / 2.0 );
/// ```
pub fn avg_seconds_to_execute( fun : fn() , total_seconds : f64 ) -> f64 {
    let mut repeat : usize = 1;
    let mut duration  = 0.0;
    while duration < total_seconds {
        let start = std::time::Instant::now();
        for _i in 0 .. repeat {
            fun();
        }
        repeat *= 2;
        duration = ( start.elapsed().as_nanos() as f64) / 1e9;
    }
    repeat = repeat / 2;
    duration / (repeat as f64)
}
