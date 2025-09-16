// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::utility::avg_seconds_to_execute;
use rustad::{
    ADfn,
    NumVec,
    start_recording,
    stop_recording,
    ad_from_value,
    ThisThreadTapePublic,
};
//
// N_SUM
// number of terms in the sum of squares
const N_SUM : usize = 10;
//
// normsq_fn
fn normsq_fn<V>()->ADfn<V>
where
    V : From<f32> + Clone + Sized + 'static + ThisThreadTapePublic ,
    for<'a> &'a V : std::ops::Mul<&'a V, Output=V> ,
    for<'a>     V : std::ops::AddAssign<&'a V> ,
{   // ax
    let zero_v : V = (0 as f32).into();
    let x          = vec![zero_v.clone() ; N_SUM];
    let ax         = start_recording(x);
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
// record_normsq_f64
fn record_normsq_f64()
{   let f  : ADfn<f64> = normsq_fn::<f64>();
    assert_eq!( f.domain_len(), N_SUM );
    assert_eq!( f.range_len(), 1 );
}
//
// record_normsq_numvec_f64
fn record_normsq_numvec_f64()
{   let f  : ADfn< NumVec<f64> > = normsq_fn::< NumVec<f64> >();
    assert_eq!( f.domain_len(), N_SUM );
    assert_eq!( f.range_len(), 1 );
}
//
// bench
fn bench( name : &String, test_case : fn() ) {
    let min_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, min_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}
//
// main
fn main() {
    bench( &"record_normsq_f64".to_string() , record_normsq_f64 );
    bench( &"record_normsq_numvec_f64".to_string() , record_normsq_numvec_f64 );
}
