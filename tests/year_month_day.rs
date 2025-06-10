// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad;

#[test]
fn test_year_month_day() {
    let year_month_day = *rustad::YEAR_MONTH_DAY;
    assert_eq!(year_month_day, "2025.6.10");
}
