// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the CompareAsLeft trait for AD types
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::{
    SimpleFloat,
    AD,
    IndexT,
    CompareAsLeft,
    CompareAsRight,
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::op::id::{
    LT_OP,
    LE_OP,
    EQ_OP,
    NE_OP,
    GE_OP,
    GT_OP,
};
//
#[cfg(doc)]
use crate::doc_generic_v;
// ---------------------------------------------------------------------------
// CompareAsLeft for AD<V>
/// CompareAsLeft trait for `AD<V>`
///
/// * Syntax : lhs.compare(&rhs)
///
///     * lhs : is the `AD<V>` left operand
///     * rhs : is the `AD<V>` or V right operand
///     * compare  : is one of `lt` , `le`, `eq`, `ne`, `ge`, `gt`
///
/// # Example
///```
/// use rustad::{
///     AD,
///     ad_from_value,
///     CompareAsLeft,
/// };
///
/// type V       = rustad::AzFloat<f64>;
/// let ax       = ad_from_value( V::from(3.0) );
/// let ay       = ad_from_value( V::from(4.0) );
/// let z        = V::from(4.0);
///
/// let ax_lt_y  = ax.left_lt(&ay);
/// assert_eq!(ax_lt_y.to_value(), V::from(1) );
///
/// let ax_ge_y  = ax.left_ge(&ay);
/// assert_eq!(ax_ge_y.to_value(), V::from(0) );
///
/// let ay_eq_z  = ay.left_eq(&z);
/// assert_eq!(ay_eq_z.to_value(), V::from(1) );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::{
///     AD,
///     AzFloat,
///     ad_from_value,
///     NumVec,
///     CompareAsLeft,
/// };
///
/// type S    = AzFloat<f32>;
/// type V    = NumVec<S>;
/// let x     = vec![ S::from(1.0), S::from(4.0) ];
/// let y     = vec![ S::from(2.0), S::from(2.0) ];
/// let x_nv  = NumVec::new(x);
/// let y_nv  = NumVec::new(y);
/// let ax    = ad_from_value(x_nv);
/// let ay    = ad_from_value(y_nv);
///
/// let ax_lt_y  = ax.left_lt(&ay);
/// let check    = NumVec::new( vec![ S::from(1), S::from(0) ] );
/// assert_eq!(ax_lt_y.to_value(), check );
///
/// let ax_gt_y  = ax.left_gt(&ay);
/// let check    = NumVec::new( vec![ S::from(0), S::from(1) ] );
/// assert_eq!(ax_gt_y.to_value(), check );
/// ```
pub fn doc_ad_compare_left() { }
//
/// Add one compare operator to the `AD<V>` class;
///
/// * V    : see [doc_generic_v]
/// * name : is the operator name; i.e., one of the following:
///     lt, le, eq, ne, ge, gt.
///
/// see [doc_ad_compare_left]
// ---------------------------------------------------------------------------
// CompareAsRight for AD<V>
/// CompareAsLeft trait for `AD<V>`
///
/// * Syntax : lhs.compare(&rhs)
///
///     * lhs : is the `AD<V>` left operand
///     * rhs : is the `AD<V>` or V right operand
///     * compare  : is one of `lt` , `le`, `eq`, `ne`, `ge`, `gt`
///
/// # Example
///```
/// use rustad::{
///     AD,
///     ad_from_value,
///     CompareAsRight,
/// };
///
/// type V       = rustad::AzFloat<f64>;
/// let ax       = ad_from_value( V::from(3.0) );
/// let z        = V::from(4.0);
///
/// let az_lt_x  = z.lt_right(&ax);
/// assert_eq!(az_lt_x.to_value(), V::from(0) );
///
/// let az_ge_x  = z.ge_right(&ax);
/// assert_eq!(az_ge_x.to_value(), V::from(1) );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::{
///     AD,
///     AzFloat,
///     ad_from_value,
///     NumVec,
///     CompareAsRight,
/// };
///
/// type S    = AzFloat<f32>;
/// type V    = NumVec<S>;
/// let x     = vec![ S::from(1.0), S::from(4.0) ];
/// let z     = vec![ S::from(2.0), S::from(2.0) ];
/// let x_nv  = NumVec::new(x);
/// let z_nv  = NumVec::new(z);
/// let ax    = ad_from_value(x_nv);
/// let z     = z_nv;
///
/// let az_lt_x  = z.lt_right(&ax);
/// let check    = NumVec::new( vec![ S::from(0), S::from(1) ] );
/// assert_eq!(az_lt_x.to_value(), check );
///
/// let az_ge_x  = z.ge_right(&ax);
/// let check    = NumVec::new( vec![ S::from(1), S::from(0) ] );
/// assert_eq!(az_ge_x.to_value(), check );
/// ```
pub fn doc_ad_compare_right() { }
// ---------------------------------------------------------------------------
//
// impl_compare_aa
macro_rules! impl_compare_aa{ ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        "& `AD<V>` num_", stringify!($name), "( & `AD<V>` )",
        "; see [doc_ad_compare_left]"
    )]
    fn [< left_ $name >](&self , rhs : &AD<V> ) -> AD<V>
    {
        // new_value
        let new_value     = self.value. [< left_ $name >] ( &rhs.value );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                let op_id = [< $name:upper _OP >];
                record_compare_aa::<V> ( tape, self, rhs, op_id )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} } }
impl<V> CompareAsLeft< AD<V> > for AD<V>
where
    V : Clone + SimpleFloat + PartialEq + CompareAsLeft + ThisThreadTape ,
{
    impl_compare_aa!( lt );
    impl_compare_aa!( le );
    impl_compare_aa!( eq );
    impl_compare_aa!( ne );
    impl_compare_aa!( ge );
    impl_compare_aa!( gt );
}
// ---------------------------------------------------------------------------
//
// impl_compare_av
macro_rules! impl_compare_av{ ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        "& `AD<V>` num_", stringify!($name), "( &V )",
        "; see [doc_ad_compare_left]"
    )]
    fn [< left_ $name >](&self , rhs : &V ) -> AD<V>
    {
        // new_value
        let new_value     = self.value. [< left_ $name >] ( rhs );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                let op_id = [< $name:upper _OP >];
                record_compare_av::<V> ( tape, self, rhs, op_id )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} } }
impl<V> CompareAsLeft<V> for AD<V>
where
    V : Clone + SimpleFloat + PartialEq + CompareAsLeft + ThisThreadTape ,
{
    impl_compare_av!( lt );
    impl_compare_av!( le );
    impl_compare_av!( eq );
    impl_compare_av!( ne );
    impl_compare_av!( ge );
    impl_compare_av!( gt );
}
// ---------------------------------------------------------------------------
//
// impl_compare_va
macro_rules! impl_compare_va{ ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        "& V" , stringify!($name), "_right( & `AD<V>` )",
        "; see [doc_ad_compare_right]"
    )]
    fn [< $name _right >](&self , rhs : &AD<Self> ) -> AD<Self>
    {
        // new_value
        let new_value     = self. [< left_ $name >] ( &rhs.value );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                let op_id = [< $name:upper _OP >];
                record_compare_va::<V> ( tape, self, rhs, op_id )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} } }
impl<V> CompareAsRight< AD<V> > for V
where
    V : Clone + SimpleFloat + PartialEq + CompareAsLeft + ThisThreadTape ,
{
    impl_compare_va!( lt );
    impl_compare_va!( le );
    impl_compare_va!( eq );
    impl_compare_va!( ne );
    impl_compare_va!( ge );
    impl_compare_va!( gt );
}
// ---------------------------------------------------------------------------
// record_compare_aa
//
fn record_compare_aa <V> (
    tape: &mut Tape<V> ,
    lhs:       &AD<V>  ,
    rhs:       &AD<V>  ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + SimpleFloat + PartialEq ,
{
    // new_tape_id, new_index, new_ad_type
    let mut new_tape_id   = 0;
    let mut new_index     = 0;
    let mut new_ad_type   = ADType::ConstantP;
    if ! tape.recording {
        return (new_tape_id, new_index, new_ad_type);
    }
    //
    // lhs_arg_type, cop_lhs, var_lhs
    let lhs_arg_type : ADType;
    let cop_lhs      : bool;
    let var_lhs      : bool;
    if lhs.tape_id != tape.tape_id {
        lhs_arg_type = ADType::ConstantP;
        cop_lhs      = true;
        var_lhs      = false;
    } else {
        debug_assert!( lhs.ad_type != ADType::ConstantP );
        lhs_arg_type = lhs.ad_type.clone();
        cop_lhs      = false;
        var_lhs      = lhs.ad_type.is_variable();
    };
    //
    // rhs_arg_type, cop_rhs, var_rhs
    let rhs_arg_type : ADType;
    let cop_rhs      : bool;
    let var_rhs      : bool;
    if rhs.tape_id != tape.tape_id {
        rhs_arg_type = ADType::ConstantP;
        cop_rhs      = true;
        var_rhs      = false;
    } else {
        debug_assert!( rhs.ad_type != ADType::ConstantP );
        rhs_arg_type = rhs.ad_type.clone();
        cop_rhs      = false;
        var_rhs      = rhs.ad_type.is_variable();
    };
    //
    if cop_lhs {
        if cop_rhs {
            return (new_tape_id, new_index, new_ad_type);
        }
    }
    //
    // new_tape_id
    new_tape_id = tape.tape_id;
    //
    if var_lhs || var_rhs {
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::Variable;
        new_index       = tape.var.n_dep + tape.var.n_dom;
        //
        // tape.var: n_dep, arg_start, arg_type
        tape.var.n_dep += 1;
        tape.var.arg_start.push( tape.var.arg_all.len() as IndexT );
        tape.var.arg_type_all.push( lhs_arg_type );
        tape.var.arg_type_all.push( rhs_arg_type );
        //
        //
        // tape.var.id_all
        tape.var.id_all.push( op_id );
        //
        // tape.cop, tape.var.arg_all
        if cop_lhs {
            tape.var.arg_all.push( tape.cop.len() as IndexT );
            tape.cop.push( lhs.value.clone() );
        } else {
            tape.var.arg_all.push( lhs.index as IndexT );
        }
        if cop_rhs {
            tape.var.arg_all.push( tape.cop.len() as IndexT );
            tape.cop.push( rhs.value.clone() );
        } else {
            tape.var.arg_all.push( rhs.index as IndexT );
        }
    } else {
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::DynamicP;
        new_index       = tape.dyp.n_dep + tape.dyp.n_dom;
        //
        // tape.dyp: n_dep, arg_start, arg_type
        tape.dyp.n_dep += 1;
        tape.dyp.arg_start.push( tape.dyp.arg_all.len() as IndexT );
        tape.dyp.arg_type_all.push( lhs_arg_type );
        tape.dyp.arg_type_all.push( rhs_arg_type );
        //
        //
        // tape.var.id_all
        tape.dyp.id_all.push( op_id );
        //
        // tape.cop, tape.dyp.arg_all
        if cop_lhs {
            tape.dyp.arg_all.push( tape.cop.len() as IndexT );
            tape.cop.push( lhs.value.clone() );
        } else {
            tape.dyp.arg_all.push( lhs.index as IndexT );
        }
        if cop_rhs {
            tape.dyp.arg_all.push( tape.cop.len() as IndexT );
            tape.cop.push( rhs.value.clone() );
        } else {
            tape.dyp.arg_all.push( rhs.index as IndexT );
        }
    }
    (new_tape_id, new_index, new_ad_type)
}
// ---------------------------------------------------------------------------
// record_compare_av
//
fn record_compare_av <V> (
    tape: &mut Tape<V> ,
    lhs:       &AD<V>  ,
    rhs:       &V      ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + SimpleFloat + PartialEq ,
{
    // new_tape_id, new_index, new_ad_type
    let mut new_tape_id   = 0;
    let mut new_index     = 0;
    let mut new_ad_type   = ADType::ConstantP;
    if ! tape.recording {
        return (new_tape_id, new_index, new_ad_type);
    }
    //
    // lhs_arg_type, cop_lhs, var_lhs
    let lhs_arg_type : ADType;
    let cop_lhs      : bool;
    let var_lhs      : bool;
    if lhs.tape_id != tape.tape_id {
        lhs_arg_type = ADType::ConstantP;
        cop_lhs      = true;
        var_lhs      = false;
    } else {
        debug_assert!( lhs.ad_type != ADType::ConstantP );
        lhs_arg_type = lhs.ad_type.clone();
        cop_lhs      = false;
        var_lhs      = lhs.ad_type.is_variable();
    };
    //
    if cop_lhs {
        return (new_tape_id, new_index, new_ad_type);
    }
    //
    // new_tape_id
    new_tape_id = tape.tape_id;
    //
    if var_lhs {
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::Variable;
        new_index       = tape.var.n_dep + tape.var.n_dom;
        //
        // tape.var: n_dep, arg_start, arg_type
        tape.var.n_dep += 1;
        tape.var.arg_start.push( tape.var.arg_all.len() as IndexT );
        tape.var.arg_type_all.push( lhs_arg_type );
        tape.var.arg_type_all.push( ADType::ConstantP );
        //
        //
        // tape.var.id_all
        tape.var.id_all.push( op_id );
        //
        // tape.cop, tape.var.arg_all
        tape.var.arg_all.push( lhs.index as IndexT );
        tape.var.arg_all.push( tape.cop.len() as IndexT );
        tape.cop.push( rhs.clone() );
    } else {
        debug_assert!( lhs.ad_type.is_dynamic() );
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::DynamicP;
        new_index       = tape.dyp.n_dep + tape.dyp.n_dom;
        //
        // tape.dyp: n_dep, arg_start, arg_type
        tape.dyp.n_dep += 1;
        tape.dyp.arg_start.push( tape.dyp.arg_all.len() as IndexT );
        tape.dyp.arg_type_all.push( lhs_arg_type );
        tape.dyp.arg_type_all.push( ADType::ConstantP );
        //
        // tape.var.id_all
        tape.dyp.id_all.push( op_id );
        //
        // tape.cop, tape.dyp.arg_all
        tape.dyp.arg_all.push( lhs.index as IndexT );
        tape.dyp.arg_all.push( tape.cop.len() as IndexT );
        tape.cop.push( rhs.clone() );
    }
    (new_tape_id, new_index, new_ad_type)
}
// ---------------------------------------------------------------------------
// record_compare_va
//
fn record_compare_va <V> (
    tape: &mut Tape<V> ,
    lhs:       &V      ,
    rhs:       &AD<V>  ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + SimpleFloat + PartialEq ,
{
    // new_tape_id, new_index, new_ad_type
    let mut new_tape_id   = 0;
    let mut new_index     = 0;
    let mut new_ad_type   = ADType::ConstantP;
    if ! tape.recording {
        return (new_tape_id, new_index, new_ad_type);
    }
    //
    // rhs_arg_type, cop_rhs, var_rhs
    let rhs_arg_type : ADType;
    let cop_rhs      : bool;
    let var_rhs      : bool;
    if rhs.tape_id != tape.tape_id {
        rhs_arg_type = ADType::ConstantP;
        cop_rhs      = true;
        var_rhs      = false;
    } else {
        debug_assert!( rhs.ad_type != ADType::ConstantP );
        rhs_arg_type = rhs.ad_type.clone();
        cop_rhs      = false;
        var_rhs      = rhs.ad_type.is_variable();
    };
    //
    if cop_rhs {
        return (new_tape_id, new_index, new_ad_type);
    }
    //
    // new_tape_id
    new_tape_id = tape.tape_id;
    //
    if var_rhs {
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::Variable;
        new_index       = tape.var.n_dep + tape.var.n_dom;
        //
        // tape.var: n_dep, arg_start, arg_type
        tape.var.n_dep += 1;
        tape.var.arg_start.push( tape.var.arg_all.len() as IndexT );
        tape.var.arg_type_all.push( ADType::ConstantP );
        tape.var.arg_type_all.push( rhs_arg_type );
        //
        //
        // tape.var.id_all
        tape.var.id_all.push( op_id );
        //
        // tape.cop, tape.var.arg_all
        tape.var.arg_all.push( tape.cop.len() as IndexT );
        tape.var.arg_all.push( rhs.index as IndexT );
        tape.cop.push( lhs.clone() );
    } else {
        debug_assert!( rhs.ad_type.is_dynamic() );
        //
        // new_ad_type, new_index
        new_ad_type     = ADType::DynamicP;
        new_index       = tape.dyp.n_dep + tape.dyp.n_dom;
        //
        // tape.dyp: n_dep, arg_start, arg_type
        tape.dyp.n_dep += 1;
        tape.dyp.arg_start.push( tape.dyp.arg_all.len() as IndexT );
        tape.dyp.arg_type_all.push( ADType::ConstantP );
        tape.dyp.arg_type_all.push( rhs_arg_type );
        //
        // tape.var.id_all
        tape.dyp.id_all.push( op_id );
        //
        // tape.cop, tape.dyp.arg_all
        tape.dyp.arg_all.push( tape.cop.len() as IndexT );
        tape.dyp.arg_all.push( rhs.index as IndexT );
        tape.cop.push( lhs.clone() );
    }
    (new_tape_id, new_index, new_ad_type)
}
