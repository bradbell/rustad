// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad;

/// add Example
fn add_example() {
    let left  = 5;
    let right = 6;
    let answer = rustad::add(left, right);

    assert_eq!(11, answer);
}

#[test]
fn main() {
    add_example();
}
