// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad;

#[test]
fn test_version() {
    let version = &*rustad::VERSION;
    assert_eq!(version, "2025.5.26");
}
