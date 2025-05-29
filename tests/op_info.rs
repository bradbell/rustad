// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad;

#[test]
fn test_op_info() {

    let op_info_vec = &*rustad::OP_INFO_VEC;
    assert_eq!( "add", op_info_vec[rustad::ADD_OP].name );
}
