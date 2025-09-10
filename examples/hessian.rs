// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Example Hessian
//
use rustad::numvec::{
    AD,
    ad_from_vector,
    NumVec,
    start_recording,
    stop_recording,
};
//
// test_hessian
// Simple case where V = f32
#[test]
fn test_hessian () {
    //
    type V     = f32;
    let nx     = 3;
    let trace  = false;
    //
    // x
    let x  : Vec<V> = vec![ 2.0 as V; nx ];
    //
    // ax
    let ax       = start_recording(x.clone());
    //
    // asum
    let mut asum : AD<V>  = AD::from(  0.0 as V );
    for j in 0 .. nx {
        let cubed  = &( &ax[j] * &ax[j] ) * &ax[j];
        asum      += &cubed;
    }
    //
    // f
    // f(x) = x[0] * x[0] * x[0] + ... + x[nx-1] * x[nx-1] * x[nx-1]
    let ay = vec![ asum ];
    let f  = stop_recording(ay);
    //
    // av
    let ax                     = start_recording(x);
    let mut av  : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av, ax, trace);
    //
    // g
    // g(x) = df/dx = [ 3 * x[0] * x[0], ..., 3 * x[nx-1] * x[nx-1] ]
    let dy  : Vec<V>  = vec![ 1.0 as V ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_one_ad(&av, ady, trace);
    let g             = stop_recording(adx);
    //
    // x
    // x[j] = j+2
    let mut x  : Vec<V> = Vec::new();
    for j in 0 .. nx {
        x.push( (j+2) as V );
    }
    //
    // v, y
    let mut v  : Vec<V> = Vec::new();
    let y               = g.forward_zero_value(&mut v, x, trace);
    for j in 0 .. nx {
        let check  = 3 * (j+2) * (j+2);
        assert_eq!( y[j], check as V );
    }
    //
    // dy
    // dy[i] = partial g[i] w.r.t x[j] = 6 * x[j]
    for j in 0 .. nx {
        let mut dx : Vec<V> = vec![ 0.0 as V; nx ];
        dx[j]               = 1.0 as V;
        let dy              = g.forward_one_value(&v, dx, trace);
        for i in 0 .. nx {
            if i == j {
                let check  = 6 * (j+2);
                assert_eq!( dy[i], check as V );
            } else {
                assert_eq!( dy[i],  0.0 as V );
            }
        }
    }
}
//
// test_numvec_hessian
// Same function where V = NumVec<f64>
#[test]
fn test_numvec_hessian () {
    //
    type F     = f64;
    type V     = NumVec<F>;
    let nx     = 3;
    let trace  = false;
    //
    // x
    let mut x  : Vec<V> = Vec::new();
    for _j in 0 .. nx {
        x.push( NumVec::new( vec![ 2.0 as F ] ) );
    }
    //
    // ax
    let ax       = start_recording(x.clone());
    //
    // asum
    let mut asum : AD<V>  = AD::from(  NumVec::from( 0.0 as F ) );
    for j in 0 .. nx {
        let cubed  = &( &ax[j] * &ax[j] ) * &ax[j];
        asum      += &cubed;
    }
    //
    // f
    // f(x) = x[0] * x[0] * x[0] + ... + x[nx-1] * x[nx-1] * x[nx-1]
    let ay = vec![ asum ];
    let f  = stop_recording(ay);
    //
    // av
    let ax                     = start_recording(x);
    let mut av  : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av, ax, trace);
    //
    // g
    // g(x) = df/dx = [ 3 * x[0] * x[0], ..., 3 * x[nx-1] * x[nx-1] ]
    let dy  : Vec<V>  = vec![ NumVec::from( 1.0 as F ) ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_one_ad(&av, ady, trace);
    let g             = stop_recording(adx);
    //
    // x
    // x[j] = [ j+1, j+2 ]
    let mut x  : Vec<V> = Vec::new();
    for j in 0 .. nx {
        x.push( NumVec::new( vec![ (j+1) as F, (j+2) as F ] ) );
    }
    //
    // v, y
    let mut v  : Vec<V> = Vec::new();
    let y               = g.forward_zero_value(&mut v, x, trace);
    for j in 0 .. nx {
        //
        let check  = 3 * (j+1) * (j+1);
        assert_eq!( y[j].vec[0], check as F );
        //
        let check  = 3 * (j+2) * (j+2);
        assert_eq!( y[j].vec[1], check as F );
    }
    //
    // dy
    // dy[i] = partial g[i] w.r.t x[j] = 6 * x[j]
    for j in 0 .. nx {
        let mut dx : Vec<V> = vec![ NumVec::from( 0.0 as F ); nx ];
        dx[j]               = NumVec::from( 1.0 as F );
        let dy              = g.forward_one_value(&v, dx, trace);
        for i in 0 .. nx {
            if i == j {
                //
                let check  = 6 * (j+1);
                assert_eq!( dy[i].vec[0], check as F );
                //
                let check  = 6 * (j+2);
                assert_eq!( dy[i].vec[1], check as F );
            } else {
                for k in 0 .. dy[i].vec.len() {
                    assert_eq!( dy[i].vec[k] ,  0.0 as F );
                }
            }
        }
    }
}
