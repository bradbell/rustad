// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::utility::avg_seconds_to_execute;
use rustad::{
    AD,
    ad_from_value,
    NumVec,
};
//
const N_SUM : usize = 10;
//
// f32
pub fn normsq_f32()
{   let mut sum_sq  = 0f32;
    for j in 0 .. N_SUM {
        sum_sq += (j as f32) * (j as f32);
    }
    assert!( (N_SUM as f32) < sum_sq );
}
//
// f64
pub fn normsq_f64()
{   let mut sum_sq  = 0f64;
    for j in 0 .. N_SUM {
        sum_sq += (j as f64) * (j as f64);
    }
    assert!( (N_SUM as f64) < sum_sq );
}
//
// AD<f64>
pub fn normsq_ad_f64()
{   let mut sum_sq  : AD<f64> = ( 0 as f64 ).into();
    for j in 0 .. N_SUM {
        let ad_j  : AD<f64> = ad_from_value(j as f64);
        sum_sq += &( &ad_j * &ad_j );
    }
    assert!( (N_SUM as f64) < sum_sq.to_value() );
}
//
// NUMVEC<f64>
pub fn normsq_nv_f64()
{   let mut sum_sq  : NumVec<f64> = (0 as f64).into();
    for j in 0 .. N_SUM {
        let nv_j  : NumVec<f64>   = (j as f64).into();
        sum_sq += &( &nv_j * &nv_j );
    }
    assert!( (N_SUM as f64) < sum_sq.vec[0] );
}
//
// AD< NUMVEC<f64> >
pub fn normsq_ad_nv_f64()
{   let mut sum_sq  : AD< NumVec<f64> > = (0 as f64).into();
    for j in 0 .. N_SUM {
        let nv_j  : AD< NumVec<f64> >   = (j as f64).into();
        sum_sq += &( &nv_j * &nv_j );
    }
    assert!( (N_SUM as f64) < sum_sq.to_value().vec[0] );
}

fn bench( name : &String, test_case : fn() ) {
    let total_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, total_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}

fn main() {
    bench( &"normsq_f32".to_string() , normsq_f32 );
    bench( &"normsq_f64".to_string() , normsq_f64 );
    bench( &"normsq_ad_f64".to_string() , normsq_ad_f64 );
    bench( &"normsq_nv_f64".to_string() , normsq_nv_f64 );
    bench( &"normsq_ad_nv_f64".to_string() , normsq_ad_nv_f64 );
}
