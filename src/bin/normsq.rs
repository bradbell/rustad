// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
use rustad::utility::avg_seconds_to_execute;
use rustad::{
    AzFloat,
    ad_from_value,
    NumVec,
};
// Define: sumsq(n) = 1 + 2^2 + ... + n^2
// Define:     S(n) = ( 2 n^3 + 3 n^2 + n )/ 6
// Claim:  sumsq(n) = S(n).
//
// Proof: Note that
// S(0)          = sumsq(0) = 0
// S(n+1) - S(n) = ( 2 ( 3 n^2 + 3n + 1) + 3 ( 2 n + 1) + 1 ) / 6
//               = n^2 + n + 2/6 + n  + 3/6  + 1/6
//               = n^2 + 2n + 1
//               = (n + 1)^2
// Suppose by induction that S(n) = sumsq(n). It follows that
// S(n+1) = sumsq(n) + (n+1)^2 = sumsq(n+1).
//
// This completes the proof of this claim (by induction).
//
// N_SUM
// number of terms in the sum of squares
const N_SUM : usize = 15;
//
// six_times_normsq
// Used to check calculation so that result get uses and is not optimized out.
fn six_times_normsq() -> usize
{   let n    = N_SUM;
    let n2   = n * n;
    let n3   = n * n * n;
    2 * n3 + 3 * n2 + n
}
//
// f32
pub fn normsq_f32()
{   let mut sumsq  = 0 as f32;
    for j in 1 .. (N_SUM+1) {
        sumsq += (j as f32) * (j as f32);
    }
    assert_eq!( 6.0 * sumsq, six_times_normsq() as f32 );
}
//
// f64
pub fn normsq_f64()
{   let mut sumsq  = 0 as f64;
    for j in 1 .. (N_SUM+1) {
        sumsq += (j as f64) * (j as f64);
    }
    assert_eq!( 6.0 * sumsq, six_times_normsq() as f64 );
}
//
// AzFloat<f64>
pub fn normsq_az_f64()
{   let mut sumsq  = AzFloat(0 as f64);
    for j in 1 .. (N_SUM+1) {
        sumsq += &( AzFloat(j as f64) * AzFloat(j as f64) );
    }
    assert_eq!( 6.0 * sumsq.to_inner(), six_times_normsq() as f64 );
}
//
// NumVec< AzFloat<f64> >
pub fn normsq_nv_az_f64()
{   let mut sumsq  = NumVec::from( AzFloat(0 as f64) );
    for j in 1 .. (N_SUM+1) {
        let nv_j  = NumVec::from( AzFloat(j as f64) );
        sumsq += &( &nv_j * &nv_j );
    }
    assert_eq!(
        6.0 * sumsq.get(0).to_inner(),
        six_times_normsq() as f64
    );
}
//
// AD< AzFloat<f64> >
pub fn normsq_ad_az_f64()
{   let mut sumsq  = ad_from_value( AzFloat( 0 as f64 ) );
    for j in 1 .. (N_SUM+1) {
        let ad_j  = ad_from_value( AzFloat(j as f64) );
        sumsq += &( &ad_j * &ad_j );
    }
    assert_eq!(
        6.0 * sumsq.to_value().to_inner(),
        six_times_normsq() as f64
    );
}
//
// AD< NumVec< AzFloat<f64> > >
pub fn normsq_ad_nv_az_f64()
{   let mut sumsq  = ad_from_value( NumVec::from( AzFloat(0 as f64) ) );
    for j in 1 .. (N_SUM+1) {
        let nv_j  = ad_from_value( NumVec::from( AzFloat(j as f64 ) ) );
        sumsq += &( &nv_j * &nv_j );
    }
    assert_eq!(
        6.0 * sumsq.to_value().get(0).to_inner(),
        six_times_normsq() as f64
    );
}

fn bench( name : &str, test_case : fn() ) {
    let min_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, min_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}

fn main() {
    bench( "normsq_f32" ,           normsq_f32 );
    bench( "normsq_f64" ,           normsq_f64 );
    bench( "normsq_az_f64" ,        normsq_az_f64 );
    bench( "normsq_nv_az_f64" ,     normsq_nv_az_f64 );
    bench( "normsq_ad_az_f64" ,     normsq_ad_az_f64 );
    bench( "normsq_ad_nv_az_f64" ,  normsq_ad_nv_az_f64 );
}
