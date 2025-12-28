// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Example of computing a Hessian
//
use rustad::{
    AzFloat,
    AD,
    ad_from_value,
    ad_from_vector,
    NumVec,
    start_recording,
    stop_recording,
};
//
// example_hessian
// Simple case where V = AzFloat<f32>
fn example_hessian () {
    //
    type V     = AzFloat<f32>;
    let nx     = 3;
    let trace  = false;
    //
    // x
    let x  : Vec<V> = vec![ V::from(2.0); nx ];
    //
    // ax
    let (_, ax)  = start_recording(None, x.clone());
    //
    // asum
    let mut asum : AD<V>  = ad_from_value(  V::from(0.0) );
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
    let (_, ax)  = start_recording(None, x);
    let (_, av)  = f.forward_var_ad(None, ax, trace);
    //
    // g
    // g(x) = df/dx = [ 3 * x[0] * x[0], ..., 3 * x[nx-1] * x[nx-1] ]
    let dy  : Vec<V>  = vec![ V::from(1.0) ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_one_ad(&av, ady, trace);
    let g             = stop_recording(adx);
    //
    // x
    // x[j] = j+2
    let mut x  : Vec<V> = Vec::new();
    for j in 0 .. nx {
        x.push( V::from(j+2) );
    }
    //
    // v, y
    let (y, v) = g.forward_var_value(None, x, trace);
    for j in 0 .. nx {
        let check  = 3 * (j+2) * (j+2);
        assert_eq!( y[j], V::from(check) );
    }
    //
    // dy
    // dy[i] = partial g[i] w.r.t x[j] = 6 * x[j]
    for j in 0 .. nx {
        let mut dx : Vec<V> = vec![ V::from(0.0); nx ];
        dx[j]               = V::from(1.0);
        let dy              = g.forward_one_value(&v, dx, trace);
        for i in 0 .. nx {
            if i == j {
                let check  = 6 * (j+2);
                assert_eq!( dy[i], V::from(check) );
            } else {
                assert_eq!( dy[i],  V::from(0.0) );
            }
        }
    }
}
//
// example_num_vec_hessian
// Same function where V = NumVec<f64>
fn example_num_vec_hessian () {
    //
    type S     = AzFloat<f64>;
    type V     = NumVec<S>;
    let nx     = 3;
    let trace  = false;
    //
    // x
    let mut x  : Vec<V> = Vec::new();
    for _j in 0 .. nx {
        x.push( NumVec::new( vec![ S::from(2.0) ] ) );
    }
    //
    // ax
    let (_, ax)  = start_recording(None, x.clone());
    //
    // asum
    let mut asum : AD<V>  = ad_from_value(  NumVec::from( S::from(0.0) ) );
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
    let (_, ax) = start_recording(None, x);
    let (_, av) = f.forward_var_ad(None, ax, trace);
    //
    // g
    // g(x) = df/dx = [ 3 * x[0] * x[0], ..., 3 * x[nx-1] * x[nx-1] ]
    let dy  : Vec<V>  = vec![ NumVec::from( S::from(1.0) ) ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_one_ad(&av, ady, trace);
    let g             = stop_recording(adx);
    //
    // x
    // x[j] = [ j+1, j+2 ]
    let mut x  : Vec<V> = Vec::new();
    for j in 0 .. nx {
        x.push( NumVec::new( vec![ S::from(j+1), S::from(j+2) ] ) );
    }
    //
    // y, v
    let (y, v)  = g.forward_var_value(None, x, trace);
    for j in 0 .. nx {
        //
        let check  = 3 * (j+1) * (j+1);
        assert_eq!( y[j].get(0), S::from(check) );
        //
        let check  = 3 * (j+2) * (j+2);
        assert_eq!( y[j].get(1), S::from(check) );
    }
    //
    // dy
    // dy[i] = partial g[i] w.r.t x[j] = 6 * x[j]
    for j in 0 .. nx {
        let mut dx : Vec<V> = vec![ NumVec::from( S::from(0.0) ); nx ];
        dx[j]               = NumVec::from( S::from(1.0) );
        let dy              = g.forward_one_value(&v, dx, trace);
        for i in 0 .. nx {
            if i == j {
                //
                let check  = 6 * (j+1);
                assert_eq!( dy[i].get(0), S::from(check) );
                //
                let check  = 6 * (j+2);
                assert_eq!( dy[i].get(1), S::from(check) );
            } else {
                for k in 0 .. dy[i].len() {
                    assert_eq!( dy[i].get(k) ,  S::from(0.0) );
                }
            }
        }
    }
}
fn main() {
    example_hessian();
    example_num_vec_hessian();
}
