// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    AzFloat,
    start_recording,
    stop_recording,
    FBinary,
};
//
// test_hypot_ca
fn test_hypot_ca() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = (&V::from(4.0)).hypot(&ax[0]);
    let ay_1         = (&V::from(5.0)).hypot(&ax[1]);
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    let check        = V::from(4.0).hypot(x[0]);
    assert_eq!(y[0], check);
    let check        = V::from(5.0).hypot(x[1]);
    assert_eq!(y[1], check);
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = ( x[0] / y[0] ) * dx[0];
    assert_eq!(dy[0], check);
    let check        = ( x[1] / y[1] ) * dx[1];
    assert_eq!(dy[1], check);
    //
    let dy           = vec![ V::from(8.0), V::from(9.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = ( x[0]/ y[0] ) * dy[0];
    assert_eq!( dx[0], check);
    let check        = ( x[1]/ y[1] ) * dy[1];
    assert_eq!( dx[1], check);
}
//
// test_hypot_ac
fn test_hypot_ac() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = (&ax[0]).hypot( &V::from(4.0) );
    let ay_1         = (&ax[1]).hypot( &V::from(5.0) );
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    assert_eq!( y[0], x[0].hypot( V::from(4.0) ) );
    assert_eq!( y[1], x[1].hypot( V::from(5.0) ) );
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = ( x[0] / y[0] ) * dx[0];
    assert_eq!(dy[0], check);
    let check        = ( x[1] / y[1] ) * dx[1];
    assert_eq!(dy[1], check);
    //
    let dy           = vec![ V::from(8.0), V::from(9.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = ( x[0]/ y[0] ) * dy[0];
    assert_eq!( dx[0], check);
    let check        = ( x[1]/ y[1] ) * dy[1];
    assert_eq!( dx[1], check);
}
//
// test_hypot_aa
fn test_hypot_aa() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0), V::from(3.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ (&ax[0]).hypot( &ax[1] ) ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), &arg_vec);
    let check        = x[0].hypot(x[1]);
    assert_eq!(y[0], check);
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    let check        = (x[0] / y[0]) * dx[0] + (x[1] / y[0]) * dx[1];
    assert_eq!(dy[0], check);
    //
    let dy           = vec![ V::from(8.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    let check        = (x[0] / y[0]) * dy[0];
    assert_eq!(dx[0], check);
    let check        = (x[1] / y[0]) * dy[0];
    assert_eq!(dx[1], check);
}
#[test]
fn hypot() {
    test_hypot_ca();
    test_hypot_ac();
    test_hypot_aa();
}
