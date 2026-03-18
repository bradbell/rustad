// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    AzFloat,
    start_recording,
    stop_recording,
    FUnary,
    FBinary,
    check_nearly_eq,
};
//
// test_atan2_ca
fn test_atan2_ca() {
    type V      = AzFloat<f64>;
    let opt_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  = vec![ V::from(1.0), V::from(2.0) ];
    let y  = vec![ V::from(3.0), V::from(4.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let az_0         = (&y[0]).atan2(&ax[0]);
    let az_1         = (&y[1]).atan2(&ax[1]);
    let az           = vec! [ az_0, az_1 ];
    let f            = stop_recording(az);
    //
    let (z, v)       = f.forward_var_value(None, x.clone(), &opt_vec);
    let check        = y[0].atan2(x[0]);
    assert_eq!(z[0], check);
    let check        = y[1].atan2(x[1]);
    assert_eq!(z[1], check);
    //
    // this check of the atan2 derivative is much different from op/atan2.rs.
    let datan    = vec![ z[0].cos().powi(2), z[1].cos().powi(2) ];
    let dratio   = vec![
        y[0].minus() / x[0].powi(2), y[1].minus() / x[1].powi(2)
    ];
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dz           = f.forward_der_value(None, &v, dx.clone(), &opt_vec);
    let check        = datan[0] * dratio[0] * dx[0];
    check_nearly_eq::<V>(&dz[0], &check, &opt_vec);
    let check        = datan[1] * dratio[1] * dx[1];
    check_nearly_eq::<V>(&dz[1], &check, &opt_vec);
    //
    let dz           = vec![ V::from(8.0), V::from(9.0) ];
    let dx           = f.reverse_der_value(None, &v, dz.clone(), &opt_vec);
    let check        = datan[0] * dratio[0] * dz[0];
    check_nearly_eq::<V>(&dx[0], &check, &opt_vec);
    let check        = datan[1] * dratio[1] * dz[1];
    check_nearly_eq::<V>(&dx[1], &check, &opt_vec);
}
//
// test_atan2_ac
fn test_atan2_ac() {
    type V      = AzFloat<f64>;
    let opt_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  = vec![ V::from(1.0), V::from(2.0) ];
    let y  = vec![ V::from(3.0), V::from(4.0) ];
    //
    let (_, ay)      = start_recording(None,  y.clone() );
    let az_0         = (&ay[0]).atan2(&x[0]);
    let az_1         = (&ay[1]).atan2(&x[1]);
    let az           = vec! [ az_0, az_1 ];
    let f            = stop_recording(az);
    //
    let (z, v)       = f.forward_var_value(None, y.clone(), &opt_vec);
    let check        = y[0].atan2(x[0]);
    assert_eq!(z[0], check);
    let check        = y[1].atan2(x[1]);
    assert_eq!(z[1], check);
    //
    // this check of the atan2 derivative is much different from op/atan2.rs.
    let datan    = vec![ z[0].cos().powi(2), z[1].cos().powi(2) ];
    let dratio   = vec![ x[0].powi(-1), x[1].powi(-1) ];
    //
    let dy : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dz           = f.forward_der_value(None, &v, dy.clone(), &opt_vec);
    let check        = datan[0] * dratio[0] * dy[0];
    check_nearly_eq::<V>(&dz[0], &check, &opt_vec);
    let check        = datan[1] * dratio[1] * dy[1];
    check_nearly_eq::<V>(&dz[1], &check, &opt_vec);
    //
    let dz           = vec![ V::from(8.0), V::from(9.0) ];
    let dy           = f.reverse_der_value(None, &v, dz.clone(), &opt_vec);
    let check        = datan[0] * dratio[0] * dz[0];
    check_nearly_eq::<V>(&dy[0], &check, &opt_vec);
    let check        = datan[1] * dratio[1] * dz[1];
    check_nearly_eq::<V>(&dy[1], &check, &opt_vec);
}
//
// test_atan2_aa
fn test_atan2_aa() {
    type V      = AzFloat<f64>;
    let opt_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  = V::from(2.0);
    let y  = V::from(3.0);
    let u  = vec![ x, y ];
    //
    let (_, au)      = start_recording(None,  u.clone() );
    let ax           = au[0].clone();
    let ay           = au[1].clone();
    let az           = vec![ (&ay).atan2(&ax) ];
    let f            = stop_recording(az);
    //
    let (z, v)       = f.forward_var_value(None, u.clone(), &opt_vec);
    let check        = y.atan2(x);
    assert_eq!(z[0], check);
    //
    // this check of the atan2 derivative is much different from op/atan2.rs.
    let datan     = z[0].cos().powi(2);
    let dratio_x  = y.minus() / x.powi(2);
    let dratio_y  = x.powi(-1);
    //
    let du : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dz           = f.forward_der_value(None, &v, du.clone(), &opt_vec);
    let check        = datan * ( dratio_x * du[0] + dratio_y * du[1] );
    check_nearly_eq::<V>(&dz[0], &check, &opt_vec);
    //
    let dz           = vec![ V::from(8.0) ];
    let du           = f.reverse_der_value(None, &v, dz.clone(), &opt_vec);
    let check        = datan * dratio_x * dz[0];
    check_nearly_eq::<V>(&du[0], &check, &opt_vec);
    let check        = datan * dratio_y * dz[0];
    check_nearly_eq::<V>(&du[1], &check, &opt_vec);
}
#[test]
fn atan2() {
    test_atan2_ca();
    test_atan2_ac();
    test_atan2_aa();
}
