// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
use std::cell::RefCell;
//
use rustad::utility::avg_seconds_to_execute;
use rustad::{
    AzFloat,
    ADfn,
    NumVec,
    start_recording,
    stop_recording,
    ad_from_value,
    ThisThreadTapePublic,
    SimpleFloat,
};
//
// V_Scalar, V_NumVec
type ScalarV = AzFloat<f64>;
type NumVecV = NumVec< AzFloat<f64> >;
//
// N_SUM
// number of terms in the sum of squares
const N_SUM : usize = 15;
//
thread_local! {
    static NORMSQ_F64 : RefCell< ADfn<ScalarV> > =
        RefCell::new( ADfn::new() );
}
//
thread_local! {
    static NORMSQ_NUMVEC_F64 : RefCell< ADfn<NumVecV> > =
        RefCell::new( ADfn::new() );
}
//
// normsq_fn
fn normsq_fn<V>()->ADfn<V>
where
    V : From<f32> + SimpleFloat + PartialEq + Clone +
        Sized + 'static + ThisThreadTapePublic ,
    for<'a> &'a V : std::ops::Mul<&'a V, Output=V> ,
    for<'a>     V : std::ops::AddAssign<&'a V> ,
{   // ax
    let zero_v : V = (0 as f32).into();
    let x          = vec![zero_v.clone() ; N_SUM];
    let (_, ax)    = start_recording(None, x);
    //
    // sumsq
    let mut sumsq  = ad_from_value(zero_v);
    for j in 0 .. ax.len() {
        sumsq += &( &ax[j] * &ax[j] );
    }
    //
    // f
    let ay  = vec![ sumsq ];
    let f   = stop_recording(ay);
    f
}
//
// six_times_normsq
// This is six times normsq when x[j] = j+1; see normsq.rs.
fn six_times_normsq() -> f64
{   let n    = N_SUM;
    let n2   = n * n;
    let n3   = n * n * n;
    ( 2 * n3 + 3 * n2 + n ) as f64
}
//
// record_normsq_scalar
fn record_normsq_scalar()
{   NORMSQ_F64.with_borrow_mut( |f_static| {
        let mut f = normsq_fn::<ScalarV>();
        f_static.swap(&mut f);
        assert_eq!( f_static.var_dom_len(), N_SUM );
        assert_eq!( f_static.rng_len(), 1 );
    } );
}
//
// record_normsq_num_vec
fn record_normsq_num_vec()
{   NORMSQ_NUMVEC_F64.with_borrow_mut( |f_static| {
        let mut f = normsq_fn::<NumVecV>();
        f_static.swap(&mut f);
        assert_eq!( f_static.var_dom_len(), N_SUM );
        assert_eq!( f_static.rng_len(), 1 );
    } );
}
//
// forward_var_normsq_scalar
fn forward_var_normsq_scalar()
{   let zero   = ScalarV::from(0.0);
    let mut x  = vec![zero; N_SUM];
    for j in 0 .. N_SUM {
        x[j] = ScalarV::from( (j + 1) as f64 );
    }
    let trace                  = false;
    let sumsq = NORMSQ_F64.with_borrow_mut( |f_static| {
        let (y, _) = f_static.forward_var_value(None, x, trace);
        y[0]
    } );
    assert_eq!(
        ScalarV::from(6.0) * sumsq,
        ScalarV::from( six_times_normsq() )
    );
}
//
// forward_var_normsq_num_vec
fn forward_var_normsq_num_vec()
{   let zero     = NumVecV::from(0.0);
    let mut x    = vec![zero; N_SUM];
    for j in 0 .. N_SUM {
        x[j] = NumVecV::from( (j+1) as f64 );
    }
    let trace                             = false;
    let sumsq = NORMSQ_NUMVEC_F64.with_borrow_mut( |f_static| {
        let (y, _) = f_static.forward_var_value(None, x, trace);
        let mut y_itr = y.into_iter();
        y_itr.next().unwrap()
    } );
    assert_eq!(
        ScalarV::from(6.0) * sumsq.get(0),
        ScalarV::from( six_times_normsq() )
    );
}
//
// bench
fn bench( name : &str, test_case : fn() ) {
    let min_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, min_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}
//
// main
fn main() {
    bench( "record_normsq_scalar" ,       record_normsq_scalar );
    bench( "record_normsq_num_vec" ,      record_normsq_num_vec );
    bench( "forward_var_normsq_scalar" ,  forward_var_normsq_scalar );
    bench( "forward_var_normsq_num_vec" , forward_var_normsq_num_vec );
}
