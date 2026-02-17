// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the NumCmp trait for AD types
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::{
    FloatCore,
    AD,
    IndexT,
    NumCmp,
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
// ---------------------------------------------------------------------------
// NumCmp for AD<V>
/// NumCmp trait for `AD<V>`
///
/// * Syntax :
///   ```text
///     res = lhs.cmp(&rhs)
///   ```
///   where either lhs or rhs has type `AD<V>` .
///
/// * V :
///   see [doc_generic_v](crate::doc_generic_v).
///
/// * lhs :
///   is the `AD<V>` or `V` left comparison operand .
/// * rhs :
///   is the `AD<V>` or `V` right comparison operand .
/// * cmp :
///   is one of `num_lt` , `num_le`, `num_eq`, `num_ne`, `num_ge`, `num_gt`
/// * res :
///   has type `AD<V>` and
///   is one (zero) if the comparison result is true (false).
///
/// # Example
///```
/// use rustad::{
///     AD,
///     ad_from_value,
///     NumCmp,
/// };
///
/// type V  = rustad::AzFloat<f64>;
/// let ax  = ad_from_value( V::from(3.0) );
/// let ay  = ad_from_value( V::from(4.0) );
/// let z   = V::from(4.0);
///
/// let ax_lt_y  = ax.num_lt(&ay);
/// assert_eq!(ax_lt_y.to_value(), V::from(1) );
///
/// let ax_ge_y  = ax.num_ge(&ay);
/// assert_eq!(ax_ge_y.to_value(), V::from(0) );
///
/// let ay_eq_z  = ay.num_eq(&z);
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
///     NumCmp,
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
/// let ax_lt_y  = ax.num_lt(&ay);
/// let check    = NumVec::new( vec![ S::from(1), S::from(0) ] );
/// assert_eq!(ax_lt_y.to_value(), check );
///
/// let ax_gt_y  = ax.num_gt(&ay);
/// let check    = NumVec::new( vec![ S::from(0), S::from(1) ] );
/// assert_eq!(ax_gt_y.to_value(), check );
/// ```
pub fn doc_num_cmp_ad() { }
// ---------------------------------------------------------------------------
//
// impl_num_cmp_aa
macro_rules! impl_num_cmp_aa{ ($name:ident, $OpId:ident) =>  {
    //
    #[doc = concat!(
        " & `AD<V>` ", stringify!($name), "( & `AD<V>` )",
        "; see [doc_num_cmp_ad]"
    )]
    fn $name(&self , rhs : &AD<V> ) -> AD<V>
    {
        // new_value
        let new_value : V = self.value.$name( &rhs.value );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                record_num_cmp_aa::<V> ( tape, self, rhs, $OpId )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} }
impl<V> NumCmp< AD<V> > for AD<V>
where
    V : Clone + FloatCore + PartialEq + ThisThreadTape ,
    V : NumCmp<V, Output = V> ,
{
    type Output = AD<V>;
    //
    impl_num_cmp_aa!( num_lt, LT_OP);
    impl_num_cmp_aa!( num_le, LE_OP );
    impl_num_cmp_aa!( num_eq, EQ_OP );
    impl_num_cmp_aa!( num_ne, NE_OP );
    impl_num_cmp_aa!( num_ge, GE_OP );
    impl_num_cmp_aa!( num_gt, GT_OP );
}
// ---------------------------------------------------------------------------
//
// impl_num_cmp_ac
macro_rules! impl_num_cmp_ac{ ($name:ident, $OpId:ident) => {
    //
    #[doc = concat!(
        "& `AD<V>` ", stringify!($name), "( &V )",
        "; see [doc_num_cmp_ad]"
    )]
    fn $name(&self , rhs : &V ) -> AD<V>
    {
        // new_value
        let new_value : V = self.value.$name( rhs );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                record_num_cmp_ac::<V> ( tape, self, rhs, $OpId )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} }
impl<V> NumCmp<V> for AD<V>
where
    V : Clone + FloatCore + PartialEq + ThisThreadTape ,
    V : NumCmp<V, Output = V> ,
{
    type Output = AD<V>;
    //
    impl_num_cmp_ac!( num_lt, LT_OP);
    impl_num_cmp_ac!( num_le, LE_OP );
    impl_num_cmp_ac!( num_eq, EQ_OP );
    impl_num_cmp_ac!( num_ne, NE_OP );
    impl_num_cmp_ac!( num_ge, GE_OP );
    impl_num_cmp_ac!( num_gt, GT_OP );
}
// ---------------------------------------------------------------------------
//
// impl_num_cmp_ca
macro_rules! impl_num_cmp_ca{ ($name:ident, $OpId:ident) => {
    //
    #[doc = concat!(
        "& V" , stringify!($name), "( & `AD<V>` )",
        "; see [doc_num_cmp_ad]"
    )]
    fn $name(&self , rhs : &AD<V> ) -> AD<V>
    {
        // new_value
        let new_value  : V = self.$name( &rhs.value );
        //
        // local_key
        let local_key : &LocalKey< RefCell< Tape<V> > > =
            ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let (new_tape_id, new_index, new_ad_type) =
            local_key.with_borrow_mut( |tape| {
                record_num_cmp_ca::<V> ( tape, self, rhs, $OpId )
            } );
        //
        // result
        AD::new(new_tape_id, new_index, new_ad_type, new_value)
    }
} }
impl<V> NumCmp< AD<V> > for V
where
    V : Clone + FloatCore + PartialEq + ThisThreadTape ,
    V : NumCmp<V, Output = V>
{
    type Output = AD<V>;
    //
    impl_num_cmp_ca!( num_lt, LT_OP);
    impl_num_cmp_ca!( num_le, LE_OP );
    impl_num_cmp_ca!( num_eq, EQ_OP );
    impl_num_cmp_ca!( num_ne, NE_OP );
    impl_num_cmp_ca!( num_ge, GE_OP );
    impl_num_cmp_ca!( num_gt, GT_OP );
}
// ---------------------------------------------------------------------------
// record_num_cmp_aa
//
fn record_num_cmp_aa <V> (
    tape: &mut Tape<V> ,
    lhs:       &AD<V>  ,
    rhs:       &AD<V>  ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + FloatCore + PartialEq ,
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
    if cop_lhs && cop_rhs {
        return (new_tape_id, new_index, new_ad_type);
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
// record_num_cmp_ac
//
fn record_num_cmp_ac <V> (
    tape: &mut Tape<V> ,
    lhs:       &AD<V>  ,
    rhs:       &V      ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + FloatCore + PartialEq ,
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
// record_num_cmp_ca
//
fn record_num_cmp_ca <V> (
    tape: &mut Tape<V> ,
    lhs:       &V      ,
    rhs:       &AD<V>  ,
    op_id:     u8      ,
) -> (usize, usize, ADType)
where
    V : Clone + FloatCore + PartialEq ,
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
