// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AD,
    IndexT,
    ad_from_value,
};
//
// V
use super::V;
//
macro_rules! eval_from_value {
    //
    (value, $value:expr) => {{
        $value
    }};
    (ad,    $value:expr) => {{
        ad_from_value( $value )
    }};
}

//
macro_rules! sumsq_forward_zero {
    ( $suffix:ident, $V:ident, $E:ty ) => { paste::paste! {
        #[doc = concat!(
            " `", stringify!($E), "` evaluation of zero order atomic sumsq; ",
        )]
        pub fn [< sumsq_forward_zero_ $suffix >] (
            var_zero     : &mut Vec<$E> ,
            domain_zero  : Vec<& $E>    ,
            _call_info   : IndexT       ,
            trace        : bool         ,
        ) -> Vec<$E>
        {   //
            // var_zero, sumsq_zero
            assert_eq!( var_zero.len(), 0 );
            let mut sumsq_zero = eval_from_value!($suffix, 0 as V);
            for j in 0 .. domain_zero.len() {
                sumsq_zero += &( domain_zero[j] * domain_zero[j] );
                var_zero.push( ( *domain_zero[j] ).clone() );
            }
            if trace {
                println!("Begin Trace: sumsq_forward_zero_value");
                print!("domain_zero = [ ");
                for j in 0 .. domain_zero.len() {
                        print!("{}, ", domain_zero[j]);
                }
                println!("]");
                println!("sumsq_zero = {}", sumsq_zero);
                println!("End Trace: sumsq_forward_zero_value");
            }
            vec![ sumsq_zero ]
        }
    } }
}
//
// sumsq_forward_value
// sumsq_forward_ad
sumsq_forward_zero!(value, V, V     );
sumsq_forward_zero!(ad,    V, AD<V> );
