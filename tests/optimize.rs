// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
/*
Test ADfn::optimize
*/
// use
use rustad::{
    AD,
    start_recording_dyp_var,
    stop_recording,
    AzFloat,
    call_atom,
};
//
mod atom_test;
//
// V
type V = AzFloat<f64>;
type W = AzFloat<f32>;
//
// compress_cop
fn compress_cop() {
    //
    // trace, n_repeat
    let trace    = false;
    let n_repeat = 5;
    //
    // four, p, x, ap, ax
    let p    = vec! [W::from(2.0) ];
    let x    = vec![ W::from(3.0) ];
    let four = W::from(4.0);
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // ay
    let mut ay : Vec< AD<W> >  = Vec::new();
    for _i in 0 .. n_repeat {
        ay.push( &ap[0] + &four );
    }
    for _i in 0 .. n_repeat {
        ay.push( &ax[0] + &four );
    }
    //
    // f, n_cop
    // 2*n_repeat constants in computation pulse Nan (which is always there).
    let mut f = stop_recording(ay);
    let n_cop = f.cop_len();
    assert_eq!( 2 * n_repeat + 1, n_cop);
    //
    // f, n_cop
    // one version of repeated constant in computation pulse the Nan
    f.optimize(trace);
    let n_cop = f.cop_len();
    assert_eq!( 2, n_cop);
    //
    // y
    let p_both       = f.forward_dyp_value(p.clone(), trace);
    let (y, _y_both) = f.forward_var_value(&p_both, x.clone(), trace);
    //
    // check
    let check = &p[0] + &four;
    for i in 0 .. n_repeat {
        assert_eq!( y[i], check );
    }
    let check = &x[0] + &four;
    for i in 0 .. n_repeat {
        assert_eq!( y[n_repeat + i], check );
    }
}
//
// compress_dyp
fn compress_dyp() {
    //
    // trace, n_repeat
    let trace    = false;
    let n_repeat = 5;
    //
    // four, p, x, ap, ax
    let p    = vec! [V::from(2.0) ];
    let x    = vec![ V::from(3.0) ];
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // ay
    let mut ay : Vec< AD<V> >  = Vec::new();
    for _i in 0 .. n_repeat {
        let aq = &ap[0] + &ap[0];
        ay.push(aq);
    }
    for _i in 0 .. n_repeat {
        let aq = &ap[0] + &ap[0];
        ay.push( &ax[0] + &aq  );
    }
    //
    // f, n_dyp
    // 2 * n_repeat dynamic parameters in computation (plus one domain dyp).
    let mut f = stop_recording(ay);
    let n_dyp = f.dyp_len();
    assert_eq!( 2 * n_repeat + 1, n_dyp);
    //
    // f, n_dyp
    // one dynamic parameter in computation plus domain dyp
    f.optimize(trace);
    let n_dyp = f.dyp_len();
    assert_eq!(2, n_dyp);
    //
    // y
    let p_both       = f.forward_dyp_value(p.clone(), trace);
    let (y, _y_both) = f.forward_var_value(&p_both, x.clone(), trace);
    //
    // check
    let check = &p[0] + &p[0];
    for i in 0 .. n_repeat {
        assert_eq!( y[i], check );
    }
    let check = &x[0] + &( &p[0] + &p[0] );
    for i in 0 .. n_repeat {
        assert_eq!( y[n_repeat + i], check );
    }
}
//
// compress_var
fn compress_var() {
    //
    // trace, n_repeat
    let trace    = false;
    let n_repeat = 5;
    //
    // four, p, x, ap, ax
    let p    = vec! [V::from(2.0) ];
    let x    = vec![ V::from(3.0) ];
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // ay
    let mut ay : Vec< AD<V> >  = Vec::new();
    for _i in 0 .. n_repeat {
        ay.push( &ap[0] + &ax[0] );
    }
    for _i in 0 .. n_repeat {
        ay.push( &ay[0] * &ax[0] );
    }
    //
    // f, n_dyp, n_var
    // 2 * n_repeat variables in computation (plus one domain variable).
    let mut f = stop_recording(ay);
    let n_dyp = f.dyp_len();
    let n_var = f.var_len();
    assert_eq!( 1, n_dyp );
    assert_eq!( 2 * n_repeat + 1, n_var);
    //
    // f, n_dyp
    // two variable in computation plus domain variable
    f.optimize(trace);
    let n_dyp = f.dyp_len();
    let n_var = f.var_len();
    assert_eq!(1, n_dyp);
    assert_eq!(3, n_var);
    //
    // y
    let p_both       = f.forward_dyp_value(p.clone(), trace);
    let (y, _y_both) = f.forward_var_value(&p_both, x.clone(), trace);
    //
    // check
    let check = &p[0] + &x[0];
    for i in 0 .. n_repeat {
        assert_eq!( y[i], check );
    }
    let check = &y[0] * &x[0];
    for i in 0 .. n_repeat {
        assert_eq!( y[n_repeat + i], check );
    }
}
//
// find_first_equal_call
fn find_first_equal_call() {
    //
    // trace
    let trace = false;
    //
    // eye_atom_id, call_info
    let eye_atom_id = atom_test::register_eye::<V>();
    let call_info   = 0;
    //
    // p, x, ap, ax
    let p    = vec![V::from(2.0), V::from(3.0) ];
    let x    = vec![V::from(4.0) ];
    let (ap, _ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // aq
    let mut aq  : Vec< AD<V> > = Vec::new();
    aq.push( &ap[0] + &ap[0] );  // q[0] = p[0] + p[0], first dependent
    aq.push( &ap[1] * &ap[1] );  // q[1] = p[1] * p[1], second dependent
    //
    // ar
    let mut ar  : Vec< AD<V> > = Vec::new();
    ar.push( &ap[0] + &ap[0] );  // r[0] = p[0] + p[0], identical to first
    ar.push( &ap[1] * &ap[1] );  // r[1] = p[1] * p[1], identical to second
    //
    // u = q
    // first call has two dependent results
    let nu = aq.len();
    let au = call_atom(nu, aq, eye_atom_id, call_info, trace);
    //
    // w = r
    // identical to first call
    let nw = ar.len();
    let aw = call_atom(nw, ar, eye_atom_id, call_info, trace);
    //
    // y = u + w
    let mut ay  : Vec< AD<V> > = Vec::new();
    ay.push( &au[0] + &aw[0] );  // y[0] = u[0] + w[0], next to last dependent
    ay.push( &au[1] * &aw[1] );  // y[1] = u[1] * w[1], last dependent
    //
    //
    // f, n_dyp, n_var
    let mut f = stop_recording(ay);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (y, _y) = f.forward_var_value(&p_, x.clone(), trace);
    let u_0     = &p[0] + &p[0];
    let u_1     = &p[1] * &p[1];
    assert_eq!(y[0], &u_0 + &u_0);
    assert_eq!(y[1], &u_1 * &u_1);
    assert_eq!( f.dyp_dep_len(), 10 );
    assert_eq!( f.var_dep_len(), 0 );
    //
    // f
    f.optimize(trace);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (y, _y) = f.forward_var_value(&p_, x.clone(), trace);
    let u_0     = &p[0] + &p[0];
    let u_1     = &p[1] * &p[1];
    assert_eq!(y[0], &u_0 + &u_0);
    assert_eq!(y[1], &u_1 * &u_1);
    assert_eq!( f.dyp_dep_len(), 6 );
    assert_eq!( f.var_dep_len(), 0 );
}
//
// find_first_equal_binary
fn find_first_equal_binary() {
    //
    // trace
    let trace = false;
    //
    // p, x, ap, ax
    let p    = vec![ W::from(2.0) ];
    let x    = vec![ W::from(3.0) ];
    let (ap, _ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // aq0, aq1, aq2, aq3
    // Optimizer should detect that aq0 and aq1 are identical.
    // GIven that, it should detect that aq2 and aq3 are identical.
    let aq0 = &ap[0] + &ap[0];   // q0 = p[0] + p[0]
    let aq1 = &ap[0] + &ap[0];   // q1 = p[0] + p[0]
    let aq2 = &ap[0] * &aq0;     // q2 = p[0] * q0
    let aq3 = &ap[0] * &aq1;     // q3 = p[0] * q1
    //
    // f
    let ay     = vec![ aq0, aq1, aq2, aq3 ];
    let mut f  = stop_recording(ay);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (y, _y) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!( y[0], &p[0] + &p[0] );
    assert_eq!( y[1], &p[0] + &p[0] );
    assert_eq!( y[2], &p[0] * &( &p[0] + &p[0] ) ) ;
    assert_eq!( y[3], &p[0] * &( &p[0] + &p[0] ) ) ;
    assert_eq!( f.dyp_dep_len(), 4 );
    assert_eq!( f.var_dep_len(), 0 );
    //
    // f
    f.optimize(trace);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (y, _y) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!( y[0], &p[0] + &p[0] );
    assert_eq!( y[1], &p[0] + &p[0] );
    assert_eq!( y[2], &p[0] * &( &p[0] + &p[0] ) ) ;
    assert_eq!( y[3], &p[0] * &( &p[0] + &p[0] ) ) ;
    assert_eq!( f.dyp_dep_len(), 2 );
    assert_eq!( f.var_dep_len(), 0 );
}
//
// an_atom_result_not_used
fn an_atom_result_not_used() {
    //
    // trace
    let trace = false;
    //
    // eye_atom_id, call_info
    let eye_atom_id = atom_test::register_eye::<V>();
    let call_info   = 0;
    //
    // p, x, ap, ax
    let p    = vec![V::from(1.0), V::from(2.0) ];
    let x    = vec![V::from(3.0), V::from(4.0) ];
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // aq
    let mut aq  : Vec< AD<V> > = Vec::new();
    aq.push( &ap[0] + &ap[0] );  // q[0] = p[0] + p[0]
    aq.push( &ap[1] * &ap[1] );  // q[1] = p[1] * p[1]
    //
    // ay
    let mut ay  : Vec< AD<V> > = Vec::new();
    ay.push( &ax[0] + &ap[0] );  // y[0] = x[0] + p[0]
    ay.push( &ax[1] * &ap[1] );  // y[1] = x[1] * p[1]
    //
    // az
    let mut az  : Vec< AD<V> > = Vec::new();
    az.push( aq[0].clone() );  // z[0] = q[0]
    az.push( aq[1].clone() );  // z[1] = q[1]
    //
    // aw
    // w = z
    let nw = az.len();
    let aw = call_atom(nw, az, eye_atom_id, call_info, trace);
    //
    // au
    let mut au : Vec< AD<V> > = Vec::new();
    au.push( aw[1].clone() ); // u[0] = w[1]
    //
    // f, n_dyp, n_var
    let mut f = stop_recording(au);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (u, _u) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!(u[0], &p[1] * &p[1] );
    //
    // n_dyp_dep, n_var_dep
    let n_dyp_dep = f.dyp_dep_len();
    let n_var_dep = f.var_dep_len();
    assert_eq!( n_dyp_dep, 4); // q[0], q[1], w[0], w[1]
    assert_eq!( n_var_dep, 2); // y[0], y[1]
    //
    // optimize
    f.optimize(trace);
    //
    // n_dyp_dep, n_var_dep
    let n_dyp_dep = f.dyp_dep_len();
    let n_var_dep = f.var_dep_len();
    assert_eq!( n_dyp_dep, 2); // q[1], w[1]
    assert_eq!( n_var_dep, 0);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (u, _u) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!(u[0], &p[1] * &p[1] );
}
//
#[test]
fn optimize() {
    compress_cop();
    compress_dyp();
    compress_var();
    find_first_equal_call();
    find_first_equal_binary();
    an_atom_result_not_used();
}
