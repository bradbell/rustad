// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    AD,
    AzFloat,
    FBinary,
    start_recording,
    stop_recording,
    pop_zero_one_message
};
//
// main
fn main() {
    //
    // V
    type V  = AzFloat<f64>;
    //
    // opt_forward
    let opt_forward : Vec<[&str; 2]> = Vec::new();
    //
    // start_message, opt_is_one
    let start_message = "value changed; see line number ";
    let mut message   = start_message.to_string();
    message          += &format!( "{}", line!() );
    message          += " in file ";
    message          += file!();
    let opt_is_one = vec![
        [ "panic",   "false" ],
        [ "message", &message ],
    ];
    //
    // p, x, ap, ax
    // Note that p < 3 and x < 3 during this this recording
    let p             = vec![ V::from(1.0) ];
    let x             = vec![ V::from(2.0) ];
    let (ap, ax)      = start_recording(Some( p.clone() ),  x.clone() );
    //
    // aq
    let ap_lt_three   = (&ap[0]).num_lt( &V::from( 3.0 ) );
    let aq            = if ap_lt_three.is_one(&opt_is_one) {
        (&ap[0]) * (&ap[0])
    } else {
        (&ap[0]) + (&ap[0])
    };
    //
    // au
    let ax_lt_three   = (&ax[0]).num_lt( &V::from( 3.0 ) );
    let au            = if ax_lt_three.is_one(&opt_is_one) {
        (&ax[0]) * (&ax[0])
    } else {
        (&ax[0]) + (&ax[0])
    };
    //
    // f
    let ay : Vec< AD<V> > = vec![ aq, au ];
    let f                 = stop_recording(ay);
    //
    // y
    // Note that p < 3 and x < 3 during this forward calculation
    let p            = vec![ V::from(2.0) ];
    let dyp_all      = f.forward_dyp_value(p.clone(), &opt_forward);
    let dyp_all      = Some(&dyp_all);
    let (y, _v_all)  = f.forward_var_value(dyp_all, x.clone(), &opt_forward);
    //
    // check
    assert_eq!( y[0], p[0] * p[0] );
    assert_eq!( y[1], x[0] * x[0] );
    let option       = pop_zero_one_message();
    match option  {
        Some(_msg) => panic!("test_is_one: expected no message"),
        None       => (),
    }
    //
    // y
    // Note that p > 3 x > 3 during this forward calculation
    let p            = vec![ V::from(4.0) ];
    let x            = vec![ V::from(5.0) ];
    let dyp_all      = f.forward_dyp_value(p.clone(), &opt_forward);
    let dyp_all      = Some(&dyp_all);
    let (y, _v_all)  = f.forward_var_value(dyp_all, x.clone(), &opt_forward);
    //
    // check
    assert_eq!( y[0], p[0] * p[0] );
    assert_eq!( y[1], x[0] * x[0] );
    //
    let option        = pop_zero_one_message();
    let total_message = "forward_var_value: is_one: ".to_string() + &message;
    match option  {
        Some(pop_msg) => assert_eq!(total_message, pop_msg),
        None          => panic!("test_is_one: expected a message"),
    }
    //
    let option        = pop_zero_one_message();
    let total_message = "forward_dyp_value: is_one: ".to_string() + &message;
    match option  {
        Some(pop_msg) => assert_eq!(total_message, pop_msg),
        None          => panic!("test_is_one: expected a message"),
    }
    //
    let option       = pop_zero_one_message();
    match option  {
        Some(_msg) => panic!("test_is_one: expected no message"),
        None       => (),
    }
}
