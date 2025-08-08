// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::gad::GAD;
use rustad::function;
use rustad::utility::avg_seconds_to_execute;

//
// vec_float2ad
fn vec_float2ad<F,U>(vec : &Vec<F> ) -> Vec< GAD<F,U> >
where
    F:        Copy,
    GAD<F,U>: From<F> ,
{
    let mut result : Vec< GAD<F, U> > = Vec::new();
    for value in vec {
        result.push( GAD::from(*value) );
    }
    result
}

// test_temp
fn test_temp() {
    use rustad::function;
    use rustad::gad::GAD;
    type AD = GAD<f32, u64>;
    let nx : usize = 4;
    let x  : Vec<f32> = vec![2.0; nx];
    let ax : Vec<AD>  = function::ad_domain(&x);
    let mut ay : Vec<AD> = Vec::new();
    ay.push( AD::from( x[0] * x[0] ) );
    for j in 1 .. nx {
        ay.push( ax[j] * ax[j] );
    }
    let f           = function::ad_fun(&ay);
    let trace       = true;
    let mut pattern = f.for_sparsity(trace);
    pattern.sort();
    assert_eq!( pattern.len(), nx-1 );
    for j in 1 .. nx {
        let j64 = j as u64;
        assert_eq!( pattern[j-1], [j64, j64] );
    }
}

// test_add_vv
fn test_add_vv() {
    type F  = f32;
    type U  = u64;
    type AD = GAD<F,U>;
    //
    let x  : Vec<F>  = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<F>  = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<F>  = vec![ 7.0, 8.0 ];
    let ax : Vec<AD> = function::ad_domain(&x);
    let ay_0    = ax[0] + ax[1];
    let ay_1    = ax[1] + ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    let trace   = false;
    let (y, v0) = f.forward_zero(&x, trace);
    let dy      = f.forward_one(&dx, &v0, trace);
    let rx      = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
    //
    assert_eq!( dy[0], dx[0] + dx[1] );
    assert_eq!( dy[1], dx[1] + dx[2] );
    //
    assert_eq!( rx[0], ry[0] );
    assert_eq!( rx[1], ry[0] + ry[1] );
    assert_eq!( rx[2], ry[1] );
}

// test_mul_vv
fn test_mul_vv() {
    type F  = f32;
    type U  = u32;
    type AD = GAD<F,U>;
    //
    let x  : Vec<F>  = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<F>  = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<F>  = vec![ 6.0, 8.0 ];
    let ax : Vec<AD>    = function::ad_domain(&x);
    let ay_0    = ax[0] * ax[1];
    let ay_1    = ax[1] * ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    let trace   = false;
    let (y, v0) = f.forward_zero(&x, trace);
    let dy      = f.forward_one(&dx, &v0, trace);
    let rx      = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] * x[1] );
    assert_eq!( y[1], x[1] * x[2] );
    //
    assert_eq!( dy[0], dx[0] * x[1] + x[0] * dx[1] );
    assert_eq!( dy[1], dx[1] * x[2] + x[1] * dx[2] );
    //
    assert_eq!( rx[0], ry[0] * x[1] );
    assert_eq!( rx[1], ry[0] * x[0] + ry[1] * x[2] );
    assert_eq!( rx[2], ry[1] * x[1] );
}

fn test_ad_mul_vv() {
    type F  = f64;
    type U  = u32;
    type AD = GAD<F,U>;
    //
    let x     : Vec<F>  = vec![ 1.0, 2.0, 3.0 ];
    let ad_x  : Vec<AD> = vec_float2ad(&x);
    //
    let dx    : Vec<F>  = vec![ 4.0, 5.0, 6.0 ];
    let ad_dx : Vec<AD> = vec_float2ad(&dx);
    //
    let ry    : Vec<F>  = vec![ 6.0, 8.0 ];
    let ad_ry : Vec<AD> = vec_float2ad(&ry);
    //
    let ax      = function::ad_domain(&x);
    let ay_0    = ax[0] * ax[1];
    let ay_1    = ax[1] * ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    //
    let trace   = false;
    //
    let (ad_y, ad_v0) = f.ad_forward_zero(&ad_x, trace);
    let ad_dy         = f.ad_forward_one(&ad_dx, &ad_v0, trace);
    let ad_rx         = f.ad_reverse_one(&ad_ry, &ad_v0, trace);
    //
    assert_eq!( ad_y[0].to_value() , x[0] * x[1] );
    assert_eq!( ad_y[1].to_value() , x[1] * x[2] );
    //
    assert_eq!( ad_dy[0].to_value() , dx[0] * x[1] + x[0] * dx[1] );
    assert_eq!( ad_dy[1].to_value() , dx[1] * x[2] + x[1] * dx[2] );
    //
    assert_eq!( ad_rx[0].to_value() , ry[0] * x[1] );
    assert_eq!( ad_rx[1].to_value() , ry[0] * x[0] + ry[1] * x[2] );
    assert_eq!( ad_rx[2].to_value() , ry[1] * x[1] );
}

fn bench( name : &String, test_case : fn() ) {
    let total_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, total_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}

fn main() {
    bench( &"test_add_vv".to_string() , test_add_vv );
    bench( &"test_mul_vv".to_string() , test_mul_vv );
    bench( &"test_ad_mul_vv".to_string() , test_ad_mul_vv );
    test_temp();
}
