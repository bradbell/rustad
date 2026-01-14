// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the numeric binary operations for the `AD<V>`.
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
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::op::id;
//
#[cfg(doc)]
use crate::doc_generic_v;
// ---------------------------------------------------------------------------
/// Binary `AD<V>` operators.
///
/// * Syntax :
/// ```text
///        az = &ax Op &ay
///        az = &ay Op &y
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Op : is the source code token for this binary operator;
/// i.e., `+` , `-` , `*` , or `/` .
///
/// * ax : left hand side `AD<V>` object
/// * ay : right hand side `AD<V>` object
/// * az : result `AD<V>` object
///
/// * x  : left hand side *V* object
/// * y  : right hand side *V* object
///
/// # Example
///```
/// use rustad::AD;
/// use rustad::ad_from_value;
///
/// type V  = rustad::AzFloat<f64>;
/// let ax  = ad_from_value( V::from(3.0) );
/// let y   = V::from(4.0);
/// let az  = &ax * &y;
/// assert_eq!( az.to_value(), V::from(12.0) );
///
/// let x   = V::from(3.0);
/// let ay  = ad_from_value( V::from(4.0) );
/// let az  = &x * &ay;
/// assert_eq!( az.to_value(), V::from(12.0) );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// use rustad::NumVec;
///
/// type S    = AzFloat<f32>;
/// type V    = NumVec<S>;
/// let x     = vec![ S::from(1.0), S::from(4.0) ];
/// let y     = vec![ S::from(2.0), S::from(2.0) ];
/// let x_nv  = NumVec::new(x);
/// let y_nv  = NumVec::new(y);
/// let ax    = ad_from_value(x_nv);
/// let ay    = ad_from_value(y_nv);
/// let az    = &ax / &ay;
/// let z_nv  = az.to_value();
/// assert_eq!( z_nv.get(0), S::from(0.5) );
/// assert_eq!( z_nv.get(1), S::from(2.0) );
/// ```
pub fn doc_ad_binary_op() { }
//
/// Add one binary operator to the `AD<V>` class;
///
/// * V : see [doc_generic_v]
/// * Name : is the operator name; i.e., Add, Sub, Mul, or Div.
/// * Op : is the operator token; i.e., +, -, *, or /.
///
/// see [doc_ad_binary_op]
macro_rules! ad_binary_op { ($Name:ident, $Op:tt) => { paste::paste! {
    // -----------------------------------------------------------------------
    fn [< record_ $Name:lower _aa >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &AD<V>  ,
        rhs:       &AD<V>  ,
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
            match id::[< $Name:upper _VV_OP >] {
                //
                id::ADD_VV_OP => {
                    // add with left operand the constant zero
                    if( lhs.value == V::zero() ) {
                        return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                    }
                },
                id::MUL_VV_OP => {
                    // multiply with left operand the constant zero
                    if( lhs.value == V::zero() ) {
                        return (new_tape_id, new_index, new_ad_type);
                    }
                    // multiply with left operand the constant one
                    if( lhs.value == V::one() ) {
                        return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                    }
                },
                /*
                Not optimized out because not a special case for AzFloat.
                id::DIV_VV_OP => {
                    // divide with left operand the constant zero
                    if( lhs.value == V::zero() ) {
                        return (new_tape_id, new_index, new_ad_type);
                    }
                },
                */
                _ => { }
            }
        } else if cop_rhs {
            match id::[< $Name:upper _VV_OP >] {
                //
                id::ADD_VV_OP => {
                    // add with right operand the constant zero
                    if( rhs.value == V::zero() ) {
                        return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                    }
                },
                id::MUL_VV_OP => {
                    // multiply with right operand the constant zero
                    if( rhs.value == V::zero() ) {
                        return (new_tape_id, new_index, new_ad_type);
                    }
                    // multiply with right operand the constant one
                    if( rhs.value == V::one() ) {
                        return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                    }
                },
                id::DIV_VV_OP => {
                    // divide with right operand the constant one
                    if( rhs.value == V::one() ) {
                        return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                    }
                },
                _ => { }
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
            if var_lhs && var_rhs {
                tape.var.id_all.push( id::[< $Name:upper _VV_OP >] );
            } else if var_lhs {
                tape.var.id_all.push( id::[< $Name:upper _VP_OP >] );
            } else {
                tape.var.id_all.push( id::[< $Name:upper _PV_OP >] );
            }
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
            tape.dyp.id_all.push( id::[< $Name:upper _PP_OP >] );
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
    //
    #[doc = concat!(
        "& `AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_binary_op]"
    )]
    impl<V> std::ops::$Name< &AD<V> > for &AD<V>
    where
        for<'a> &'a V: std::ops::$Name<&'a V, Output=V>,
        V    : Clone + SimpleFloat + PartialEq + crate::ThisThreadTapePublic ,
    {   type Output = AD<V>;
        //
        fn [< $Name:lower >](self , rhs : &AD<V> ) -> AD<V>
        {
            // new_value
            let new_value     = &self.value  $Op &rhs.value;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // new_tape_id, new_index, new_ad_type
            let (new_tape_id, new_index, new_ad_type) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower _aa >]::<V> ( tape, self, rhs )
            );
            //
            // result
            AD::new(new_tape_id, new_index, new_ad_type, new_value)
        }
    }
    // -----------------------------------------------------------------------
    fn [< record_ $Name:lower _av >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &AD<V>  ,
        rhs:       &V      ,
    ) -> (usize, usize, ADType)
    where
        V : Clone + SimpleFloat + PartialEq,
    {
        // new_tape_id, new_index, new_ad_type, cop_lhs
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
        match id::[< $Name:upper _VV_OP >] {
            //
            id::ADD_VV_OP => {
                // add with right operand the constant zero
                if( *rhs == V::zero() ) {
                    return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                }
            },
            id::MUL_VV_OP => {
                // multiply with right operand the constant zero
                if( *rhs == V::zero() ) {
                    return (new_tape_id, new_index, new_ad_type);
                }
                // multiply with right operand the constant one
                if( *rhs == V::one() ) {
                    return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                }
            },
            id::DIV_VV_OP => {
                // divide with right operand the constant one
                if( *rhs == V::one() ) {
                    return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                }
            },
            _ => { }
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
            // tape.var.id_all
            tape.var.id_all.push( id::[< $Name:upper _VP_OP >] );
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
            // tape.dyp.id_all
            tape.dyp.id_all.push( id::[< $Name:upper _PP_OP >] );
            //
            // tape.cop, tape.dyp.arg_all
            tape.dyp.arg_all.push( lhs.index as IndexT );
            tape.dyp.arg_all.push( tape.cop.len() as IndexT );
            tape.cop.push( rhs.clone() );
        }
        (new_tape_id, new_index, new_ad_type)
    }
    //
    #[doc = concat!(
        "& `AD<V>` ", stringify!($Op), " & V`",
        "; see [doc_ad_binary_op]"
    )]
    impl<V> std::ops::$Name< &V> for &AD<V>
    where
        for<'a> &'a V: std::ops::$Name<&'a V, Output=V>,
        V            : Clone + SimpleFloat + PartialEq +
                       crate::ThisThreadTapePublic ,
    {   type Output = AD<V>;
        //
        fn [< $Name:lower >](self , rhs : &V ) -> AD<V>
        {
            // new_value
            let new_value     = &self.value  $Op &rhs;
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // new_tape_id, new_index, new_ad_type
            let (new_tape_id, new_index, new_ad_type) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower _av >]::<V> ( tape, self, rhs )
            );
            //
            // result
            AD::new(new_tape_id, new_index, new_ad_type, new_value)
        }
    }
} } }
//
ad_binary_op!(Add, +);
ad_binary_op!(Sub, -);
ad_binary_op!(Mul, *);
ad_binary_op!(Div, /);
// ---------------------------------------------------------------------------
/// Compound Assignment `AD<V>` operators.
///
/// Syntax :
/// ```text
///     ax Op &ay
///     ax Op &y
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Op : is the source code token for this binary operator;
/// i.e., `+=` , `-=` , `*=` , or `/=` .
///
/// * ax : left hand side `AD<V>` object.
/// * ay : right hand size `AD<V>` object
/// * y  : right hand size *V* object
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
///
/// type V       = rustad::AzFloat<f64>;
/// let mut ax   = ad_from_value( V::from(3.0) );
/// let y        = V::from(4.0);
/// ax          -= &y;
/// assert_eq!( ax.to_value(), V::from(-1.0) );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// use rustad::NumVec;
///
/// type S     = AzFloat<f32>;
/// type V     = NumVec<S>;
/// let x      = vec![ S::from(1.0), S::from(4.0) ];
/// let y      = vec![ S::from(2.0), S::from(2.0) ];
/// let x_nv   = NumVec::new(x);
/// let y_nv   = NumVec::new(y);
/// let mut ax = ad_from_value(x_nv);
/// let ay     = ad_from_value(y_nv);
/// ax         *= &ay;
/// let x_nv   = ax.to_value();
/// assert_eq!( x_nv.get(0), S::from(2.0) );
/// assert_eq!( x_nv.get(1), S::from(8.0) );
/// ```
pub fn doc_ad_compound_op() { }
//
/// Add one compound assignment operator to the `AD<V>` class;
///
/// * V : see [doc_generic_v]
///
/// * Name : is the operator name with Assign at end;
/// i.e., Add, Sub, Mul, or Div.
///
/// * Op : is the operator token; i.e., +=, -=, *=, or /= .
///
/// see [doc_ad_compound_op]
macro_rules! ad_compound_op { ($Name:ident, $Op:tt) => { paste::paste! {
    //
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_compound_op]"
    )]
    impl<V> std::ops::[< $Name Assign >] < &AD<V> > for AD<V>
    where
        V: Clone + SimpleFloat + PartialEq +
            for<'a> std::ops::[< $Name Assign >] <&'a V> +
            crate::ThisThreadTapePublic  ,
    {   //
        fn [< $Name:lower _assign >] (&mut self, rhs : &AD<V> )
        {   //
            //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // tape, self.tape_id, self.index
            let (new_tape_id, new_index, new_ad_type) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower _aa >]::<V> ( tape, self, rhs )
            );
            //
            // self
            self.tape_id   = new_tape_id;
            self.index     = new_index;
            self.ad_type   = new_ad_type;
            //
            self.value $Op &rhs.value;
        }
    }
    // ------------------------------------------------------------------------
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & V; see [doc_ad_compound_op]"
    )]
    impl<V> std::ops::[< $Name Assign >] <&V> for AD<V>
    where
        V: Clone + SimpleFloat + PartialEq +
            for<'a> std::ops::[< $Name Assign >] <&'a V> +
            crate::ThisThreadTapePublic  ,
    {   //
        fn [< $Name:lower _assign >] (&mut self, rhs : &V)
        {   //
            // local_key
            let local_key : &LocalKey< RefCell< Tape<V> > > =
                ThisThreadTape::get();
            //
            // tape, self.tape_id, self.index
            let (new_tape_id, new_index, new_ad_type) =
                local_key.with_borrow_mut( |tape|
                    [< record_ $Name:lower _av >]::<V> ( tape, self, rhs )
            );
            //
            // self
            self.tape_id   = new_tape_id;
            self.index     = new_index;
            self.ad_type   = new_ad_type;
            //
            self.value $Op &rhs;
        }
    }
} } }
//
ad_compound_op!(Add, +=);
ad_compound_op!(Sub, -=);
ad_compound_op!(Mul, *=);
ad_compound_op!(Div, /=);
// ---------------------------------------------------------------------------
// record_value_op_ad!
//
/// Create function that records
/// one binary operation where lhs is *V* and rhs is `AD<V>` .
///
/// * Name         : is the operator name; i.e., Add, Sub, Mul, or Div.
///
/// * Op           : is the operator token; i.e., +, -, *, or /.
///
/// * Function Name: `record_value_` *name* `_ad` where *name* is
///  a lower case version of Name.
///
macro_rules! record_value_op_ad{ ($Name:ident, $Op:tt) => { paste::paste! {
    #[doc = concat!( "record one ", stringify!($Name),
        " where lhs is a value and rhs is a variable"
    ) ]
    pub(crate) fn [< record_value_ $Name:lower _ad >]<V> (
        tape: &mut Tape<V> ,
        lhs:       &V      ,
        rhs:       &AD<V>  ,
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
        match id::[< $Name:upper _VV_OP >] {
            //
            id::ADD_VV_OP => {
                // add with left operand the constant zero
                if( *lhs == V::zero() ) {
                    return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                }
            },
            id::MUL_VV_OP => {
                // multiply with left operand the constant zero
                if( *lhs == V::zero() ) {
                    return (new_tape_id, new_index, new_ad_type);
                }
                // multiply with left operand the constant one
                if( *lhs == V::one() ) {
                    return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                }
            },
            id::DIV_VV_OP => {
                // divide with left operand the constant zero
                if( *lhs == V::zero() ) {
                    return (new_tape_id, new_index, new_ad_type);
                }
            },
            _ => { }
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
            // tape.var.id_all
            tape.var.id_all.push( id::[< $Name:upper _PV_OP >] );
            //
            // tape.var.all_arg, tape.cop
            tape.var.arg_all.push( tape.cop.len() as IndexT );
            tape.var.arg_all.push( rhs.index as IndexT );
            tape.cop.push( lhs.clone() );
        } else {
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
            // tape.dyp.id_all
            tape.dyp.id_all.push( id::[< $Name:upper _PP_OP >] );
            //
            // tape.dyp.all_arg, tape.cop
            tape.dyp.arg_all.push( tape.cop.len() as IndexT );
            tape.dyp.arg_all.push( rhs.index as IndexT );
            tape.cop.push( lhs.clone() );
        }
        (new_tape_id, new_index, new_ad_type)
    }
} } }
record_value_op_ad!(Add, +=);
record_value_op_ad!(Sub, -=);
record_value_op_ad!(Mul, *=);
record_value_op_ad!(Div, /=);
// ---------------------------------------------------------------------------
// impl_value_op_ad!
//
// If you try to make this implementation generic w.r.t V,
// you get a message saying that f32 and f64 must be covered
// because they are not local types.
//
/// Implement one binary `AD<V>` operator where lhs is a *V* object.
///
/// * V : see [doc_generic_v]
/// * Name : is the operator name; i.e., Add, Sub, Mul, or Div.
/// * Op : is the operator token; i.e., +, -, *, or /.
///
/// If *V* is the only argument to this macro, it will invoke itself
/// with *Op* equal to +, -, *, / and the corresponding *Name* .
///
/// see [doc_ad_binary_op]
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
///     use crate::ad::AD;
/// ```
macro_rules! impl_value_op_ad{
    ($V:ty)                      => {
        crate::ad::binary::impl_value_op_ad!($V, Add, +);
        crate::ad::binary::impl_value_op_ad!($V, Sub, -);
        crate::ad::binary::impl_value_op_ad!($V, Mul, *);
        crate::ad::binary::impl_value_op_ad!($V, Div, /);
    };
    ($V:ty, $Name:ident, $Op:tt) => { paste::paste! {
        #[doc =
        "see [doc_ad_binary_op](crate::ad::binary::doc_ad_binary_op)"
        ]
        impl std::ops::$Name< &AD<$V> > for & $V
        where
            for <'a> &'a $V : std::ops::$Name<&'a $V, Output=$V>,
        {   type Output = AD<$V>;
            //
            #[ doc = concat!(
                "compute & `", stringify!($V), "` ",
                stringify!($Op), " & `AD<", stringify!($f1), ">` "
            ) ]
            fn [< $Name:lower >]
                (self , rhs : &AD<$V>
            ) -> AD<$V> {
                //
                // new_value
                let new_value = self $Op &rhs.value;
                //
                // local_key
                let local_key : &LocalKey<
                    RefCell< crate::tape::Tape<$V> >
                > = crate::tape::sealed::ThisThreadTape::get();
                //
                // new_tape_id, new_index
                let (new_tape_id, new_index, new_ad_type) =
                local_key.with_borrow_mut( |tape|
                    crate::ad::binary::[< record_value_ $Name:lower _ad >]::<$V>
                            ( tape, &self, &rhs )
                );
                //
                // result
                AD::new(new_tape_id, new_index, new_ad_type, new_value)
            }
        }
    } }
}
pub(crate) use impl_value_op_ad;
