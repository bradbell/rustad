// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell

use rustad;

#[test]
fn year_month_day() {
    let year_month_day = rustad::YEAR_MONTH_DAY;
    assert_eq!(year_month_day, "2026.1.17");
}
