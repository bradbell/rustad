// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use super::*;

fn test_1() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_2() {
    let version = &*VERSION;
    assert_eq!(version, "2025.5.26");
}
