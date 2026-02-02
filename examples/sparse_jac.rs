// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2022-26 Bradley M. Bell
//
// for i = 0, ... , n-1, f_i(x) = x[i] * x[i+1]
//
use rustad::{
    AzFloat,
    AD,
    coloring,
    start_recording,
    stop_recording,
};
//
type V = AzFloat<f64>;
//
fn main () {
    //
    // n, m, trace
    let n          = 5;
    let m          = n - 1;
    let trace      = false;
    //
    // f
    let zero_v : V = 0.into();
    let x          = vec![zero_v; n];
    let (_ap, ax)  = start_recording(None, x);
    let mut ay : Vec< AD<V> > = Vec::with_capacity(m);
    for i in 0 .. m {
        ay.push( &ax[i] * &ax[i+1] );
    }
    let f = stop_recording(ay);
    //
    // pattern
    let compute_dyp = false;
    let mut pattern = f.for_sparsity(trace, compute_dyp);
    assert_eq!( pattern.len(), 2 * m );
    //
    // x
    let x : Vec<V>     = (0 .. n).map( |j| V::from(j+1) ).collect();
    let (_y, var_both) = f.forward_var_value(None, x.clone(), trace);
    //
    // -----------------------------------------------------------------------
    // for_sparse_jac_value
    //
    // color_vec, n_color
    let color_vec = coloring(m, n, &pattern, &pattern);
    let n_color   = color_vec.iter().filter( |&k| k < &n ).max().unwrap() + 1;
    assert_eq!( n_color, 2 );
    //
    // jacobian
    let jacobian = f.for_sparse_jac_value(
        None, &var_both, &pattern, &color_vec, trace
    );
    //
    // row_major
    let mut row_major : Vec<usize> = (0 .. pattern.len()).collect();
    row_major.sort_by_key( |&ell| pattern[ell] );
    //
    for i in 0 .. m {
        //
        let ell = row_major[2 * i];
        assert_eq!( pattern[ell],  [i, i] );
        assert_eq!( jacobian[ell], x[i+1] );
        //
        let ell = row_major[2 * i + 1];
        assert_eq!( pattern[ell],  [i, i+1] );
        assert_eq!( jacobian[ell], x[i] );
    }
    //
    // -----------------------------------------------------------------------
    // rev_sparse_jac_value
    //
    // pattern
    // transpose this sparsity pattern
    for pair in pattern.iter_mut() {
        *pair = [ pair[1], pair[0] ];
    }
    //
    // color_vec, n_color
    let color_vec = coloring(n, m, &pattern, &pattern);
    let n_color   = color_vec.iter().filter( |&k| k < &n ).max().unwrap() + 1;
    assert_eq!( n_color, 2 );
    //
    // jacobian
    let jacobian = f.rev_sparse_jac_value(
        None, &var_both, &pattern, &color_vec, trace
    );
    //
    // col_major
    let mut col_major : Vec<usize> = (0 .. pattern.len()).collect();
    col_major.sort_by_key( |&ell| { let [i,j] = pattern[ell]; [j, i] } );
    //
    for i in 0 .. m {
        //
        let ell = col_major[2 * i];
        assert_eq!( pattern[ell],  [i, i] );
        assert_eq!( jacobian[ell], x[i+1] );
        //
        let ell = col_major[2 * i + 1];
        assert_eq!( pattern[ell],  [i+1, i] );
        assert_eq!( jacobian[ell], x[i] );
    }
}
