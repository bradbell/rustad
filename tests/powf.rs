// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    FBinary,
    AzFloat,
    start_recording,
    stop_recording,
    FConst,
    FUnary,
    check_nearly_eq,
};
//
// test_powf_ca
fn test_powf_ca() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = (&V::from(4.0)).powf(&ax[0]);
    let ay_1         = (&V::from(5.0)).powf(&ax[1]);
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    let check        = V::from(4.0).powf(x[0]);
    check_nearly_eq::<V>(&y[0], &check, &arg_vec);
    let check        = V::from(5.0).powf(x[1]);
    check_nearly_eq::<V>(&y[1], &check, &arg_vec);
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = y[0] * V::from(4.0).ln() * dx[0];
    check_nearly_eq::<V>( &dy[0], &check, &arg_vec);
    let check        = y[1] * V::from(5.0).ln() * dx[1];
    check_nearly_eq::<V>( &dy[1], &check, &arg_vec);
    //
    let dy           = vec![ V::from(8.0), V::from(9.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = y[0] * V::from(4.0).ln() * dy[0];
    assert_eq!( dx[0], check);
    let check        = y[1] * V::from(5.0).ln() * dy[1];
    assert_eq!( dx[1], check);
}
//
// test_powf_ac
fn test_powf_ac() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = (&ax[0]).powf( &V::from(4.0) );
    let ay_1         = (&ax[1]).powf( &V::from(5.0) );
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    assert_eq!( y[0], x[0].powf( V::from(4.0) ) );
    assert_eq!( y[1], x[1].powf( V::from(5.0) ) );
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = V::from(4.0) * (&x[0]).powi(3) * dx[0];
    assert_eq!( dy[0], check);
    let check        = V::from(5.0) * (&x[1]).powi(4) * dx[1];
    assert_eq!( dy[1], check);
    //
    let dy           = vec![ V::from(8.0), V::from(9.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = V::from(4.0) * (&x[0]).powi(3) * dy[0];
    assert_eq!( dx[0], check);
    let check        = V::from(5.0) * (&x[1]).powi(4) * dy[1];
    assert_eq!( dx[1], check);
}
//
// test_powf_aa
fn test_powf_aa() {
    type V      = AzFloat<f64>;
    let one     = V::one();
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0), V::from(3.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ (&ax[0]).powf( &ax[1] ) ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    let check        = x[0].powf(x[1]);
    check_nearly_eq::<V>( &y[0], &check, &arg_vec);
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = x[1] * x[0].powf(x[1] - one) * dx[0]
                     + y[0] * (&x[0]).ln() * dx[1];
    check_nearly_eq::<V>( &dy[0], &check, &arg_vec );
    //
    let dy           = vec![ V::from(8.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = x[1] * x[0].powf(x[1] - one) * dy[0];
    check_nearly_eq::<V>( &dx[0], &check, &arg_vec );
    let check        = y[0] * (&x[0]).ln() * dy[0];
    check_nearly_eq::<V>( &dx[1], &check, &arg_vec );
}

#[test]
fn powf() {
    test_powf_ca();
    test_powf_ac();
    test_powf_aa();
}
