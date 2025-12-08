// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the automatic differentiation class `AD<V>`.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::IndexT;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::op::id;
// ---------------------------------------------------------------------------
//
// ADType
/// The AD types satisfy the following order:
/// constants < dynamic parameters < variables.
///
/// If a result depends on two arguments, the type of the result is the
/// maximum of the type of its arguments.
/// The value Empty is greater than any other type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ADType {
    //
    // ConstantP
    /// An AD object is a constant parameter if
    /// it does not depend on the value of
    /// the domain variables or domain dynamic parameters.
    ConstantP,
    //
    // DynamicP
    /// An AD object is a dynamic parameter if
    /// it depends (does not depend) on the value of the
    /// domain dynamic parameters (domain variables).
    DynamicP,
    //
    // Variable
    /// An AD object is a variable if
    /// it depends on the value of the domain variables.
    Variable,
    //
    // Empty
    /// This is used for the case where there is no information in this value
    Empty,
}
impl ADType {
    //
    /// is a constant parameter
    pub fn is_constant(&self) -> bool
    {   *self == ADType::ConstantP }
    //
    /// is a dynamic parameter
    pub fn is_dynamic(&self) -> bool
    {   *self == ADType::DynamicP }
    //
    /// is a parameter
    pub fn is_parameter(&self) -> bool
    {   *self == ADType::ConstantP ||  *self == ADType::DynamicP }
    //
    /// is a variable
    pub fn is_variable(&self) -> bool
    {   *self == ADType::Variable }
}
///
#[test]
fn test_ad_type() {
    assert!( ADType::ConstantP < ADType::DynamicP );
    assert!( ADType::DynamicP  < ADType::Variable );
    assert!( ADType::Variable  < ADType::Empty );
}
// ---------------------------------------------------------------------------
/// Documentation for the rustad generic type parameter V.
///
/// The generic parameter *V* , in ``AD`` < *V* > and other generic types ,
/// is the type used for calculating values.
/// It does not have dependency information that represents
/// how each value is related to the domain variables (independent variables).
pub fn doc_generic_v() {}
//
// AD
/// AD acts like V but in addition can record a function evaluation.
///
/// * V : see [doc_generic_v]
///
#[derive(Clone, Debug)]
pub struct AD<V> {
    //
    // tape_id
    ///
    /// This is the tape_id that the value of index below corresponds to.
    /// 1.  The tape_id zero never gets recorded.
    ///     The value of index and ad_type do not matter for this case.
    /// 2.  This object is a constant parameter if its tape_id is different
    ///     from the tape_id for this thread's tape.
    ///     The value of index and ad_type do not matter for this case.
    pub(crate) tape_id   : usize,
    //
    // index
    /// If this AD object's tape_id is the same as this thread's tape_id,
    /// *index* is the index in this thread's tape for this AD object.
    pub(crate) index : usize,
    //
    // ad_type
    /// If this AD object's tape_id is the same as this thread's tape_id,
    /// *ad_type* is Variable or DynamicP and is the type of this AD object.
    pub(crate) ad_type : ADType,
    //
    // value
    /// is the value of this AD object.
    pub(crate) value : V,
}
//
// new
impl<V> AD<V> {
    //
    /// Create an arbitrary new AD object.
    ///
    /// * new_tape_id : is the [AD::tape_id] for the new object.
    ///
    /// * new_index : is the [AD::index] for the new object.
    ///
    /// *ad_type : is the [AD::ad_type] for the new object.
    ///
    /// * new_value : is the [AD::value] for the new object.
    pub(crate) fn new(
        new_tape_id   : usize,
        new_index     : usize,
        new_ad_type   : ADType,
        new_value     : V,
    )-> Self {
        Self {
            tape_id   : new_tape_id,
            index     : new_index,
            ad_type   : new_ad_type,
            value     : new_value,
        }
    }
}
//
// to_value
impl<V> AD<V> {
    //
    /// Convert an AD object to a value
    /// (its the variable information is lost).
    ///
    /// **See Also** : example in [ad_from_value]
    ///
    /// # Example using NumVec
    /// ```
    /// use rustad::AD;
    /// use rustad::ad_from_value;
    /// use rustad::NumVec;
    /// type V  = rustad::AzFloat<f32>;
    /// let v   = vec![ V::from(2.0), V::from(3.0) ];
    /// let nv  = NumVec::new(v);
    /// let av  = ad_from_value(nv);
    /// let nv  = av.to_value();
    /// assert_eq!( nv.get(0), V::from(2.0) );
    /// assert_eq!( nv.get(1), V::from(3.0) );
    /// ```
    pub fn to_value(self) -> V {
        self.value
    }
}
// ---------------------------------------------------------------------------
// Display
//
/// Display only shows the value and ignores the variable information.
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// type V  = rustad::AzFloat<f64>;
/// let x   = V::from(5.0);
/// let ax  = ad_from_value(x);
/// let s   = format!( "{ax}" );
/// assert_eq!(s, "5");
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::ad_from_value;
/// use rustad::NumVec;
/// type S     = AzFloat<f32>;
/// type V     = NumVec<S>;
/// let x      = vec![ S::from(5.0), S::from(6.0) ];
/// let x_nv   = NumVec::new(x);
/// let ax     = ad_from_value(x_nv);
/// let s      = format!( "{ax}" );
/// assert_eq!(s, "[ 5, 6, ]");
/// ```
impl<V : std::fmt::Display> std::fmt::Display for AD<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
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
//
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
        V : Clone + From<f32> + PartialEq ,
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
                    if( lhs.value == V::from(0f32) ) {
                        return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                    }
                },
                id::MUL_VV_OP => {
                    // multiply with left operand the constant zero
                    if( lhs.value == V::from(0f32) ) {
                        return (new_tape_id, new_index, new_ad_type);
                    }
                    // multiply with left operand the constant one
                    if( lhs.value == V::from(1f32) ) {
                        return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                    }
                },
                /*
                Not optimized out because not a special case for AzFloat.
                id::DIV_VV_OP => {
                    // divide with left operand the constant zero
                    if( lhs.value == V::from(0f32) ) {
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
                    if( rhs.value == V::from(0f32) ) {
                        return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                    }
                },
                id::MUL_VV_OP => {
                    // multiply with right operand the constant zero
                    if( rhs.value == V::from(0f32) ) {
                        return (new_tape_id, new_index, new_ad_type);
                    }
                    // multiply with right operand the constant one
                    if( rhs.value == V::from(1f32) ) {
                        return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                    }
                },
                id::DIV_VV_OP => {
                    // divide with right operand the constant one
                    if( rhs.value == V::from(1f32) ) {
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
        V    : Clone + From<f32> + PartialEq + crate::ThisThreadTapePublic ,
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
        V : Clone + From<f32> + PartialEq,
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
                if( *rhs == V::from(0f32) ) {
                    return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                }
            },
            id::MUL_VV_OP => {
                // multiply with right operand the constant zero
                if( *rhs == V::from(0f32) ) {
                    return (new_tape_id, new_index, new_ad_type);
                }
                // multiply with right operand the constant one
                if( *rhs == V::from(1f32) ) {
                    return (lhs.tape_id, lhs.index, lhs.ad_type.clone());
                }
            },
            id::DIV_VV_OP => {
                // divide with right operand the constant one
                if( *rhs == V::from(1f32) ) {
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
        V            : Clone + From<f32> + PartialEq +
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
//
/// see [doc_ad_compound_op]
macro_rules! ad_compound_op { ($Name:ident, $Op:tt) => { paste::paste! {
    //
    #[doc = concat!(
        "`AD<V>` ", stringify!($Op), " & `AD<V>`",
        "; see [doc_ad_compound_op]"
    )]
    impl<V> std::ops::[< $Name Assign >] < &AD<V> > for AD<V>
    where
        V: Clone + From<f32> + PartialEq +
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
        V: Clone + From<f32> + PartialEq +
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
        V : Clone + From<f32> + PartialEq ,
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
                if( *lhs == V::from(0f32) ) {
                    return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                }
            },
            id::MUL_VV_OP => {
                // multiply with left operand the constant zero
                if( *lhs == V::from(0f32) ) {
                    return (new_tape_id, new_index, new_ad_type);
                }
                // multiply with left operand the constant one
                if( *lhs == V::from(1f32) ) {
                    return (rhs.tape_id, rhs.index, rhs.ad_type.clone());
                }
            },
            id::DIV_VV_OP => {
                // divide with left operand the constant zero
                if( *lhs == V::from(0f32) ) {
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
        crate::ad::impl_value_op_ad!($V, Add, +);
        crate::ad::impl_value_op_ad!($V, Sub, -);
        crate::ad::impl_value_op_ad!($V, Mul, *);
        crate::ad::impl_value_op_ad!($V, Div, /);
    };
    ($V:ty, $Name:ident, $Op:tt) => { paste::paste! {
        #[doc =
        "see [doc_ad_binary_op](crate::ad::doc_ad_binary_op)"
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
                        crate::ad::[< record_value_ $Name:lower _ad >]::<$V>
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
// ---------------------------------------------------------------------------
// ad_from_value
/// Convert a value to an AD object with no function information;
/// i.e., a constant parameter.
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// type V  = rustad::AzFloat<f64>;
/// let x   = V::from(3.0);
/// let ax  = ad_from_value(x);
/// assert_eq!( ax.to_value(), V::from(3.0) );
/// ```
pub fn ad_from_value<V>(value : V) -> AD<V> {
    let tape_id   = 0;
    let index     = 0;
    let ad_type   = ADType::ConstantP;
    AD::new(tape_id, index, ad_type, value)
}
// ---------------------------------------------------------------------------
// ad_from_vector
/// Convert a vector to an vector of AD objects with no function information;
/// i.e., a vector of constant parameters.
///
/// **See Also** : example in [ad_from_value]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_vector;
/// type V   = rustad::AzFloat<f32>;
/// let x    = vec![ V::from(3.0), V::from(4.0) ];
/// let ax   = ad_from_vector(x);
/// assert_eq!( ax[0].clone().to_value(), V::from(3.0) );
/// assert_eq!( ax[1].clone().to_value(), V::from(4.0) );
/// ```
pub fn ad_from_vector<V> ( vec : Vec<V> ) -> Vec< AD<V> > {
    assert_ne!( vec.len() , 0 );
    let avec      = vec.into_iter().map( |value| {
        let tape_id   = 0;
        let index     = 0;
        let ad_type   = ADType::ConstantP;
        AD::new(tape_id, index, ad_type, value)
    } ).collect();
    avec
}
// ---------------------------------------------------------------------------
// ad_to_vector
/// Convert a vector of AD object to a vector of values
/// (any variable information is lost).
///
/// **See Also** : example in [ad_from_vector], [AD::to_value]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::ad_from_value;
/// use rustad::ad_to_vector;
/// type V    = rustad::AzFloat<f64>;
/// let ax    = vec![ ad_from_value(V::from(3)), ad_from_value(V::from(4)) ];
/// let y     = ad_to_vector(ax);
/// assert_eq!( y , vec![ V::from(3), V::from(4) ] );
/// ```
///
/// # Example using NumVec
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::NumVec;
/// use rustad::ad_from_vector;
/// use rustad::ad_to_vector;
/// type S     = AzFloat<f32>;
/// type V     = NumVec<S>;
/// let v_0    = vec![ S::from(2.0), S::from(3.0) ];
/// let nv_0   = NumVec::new(v_0);
/// let v_1    = vec![ S::from(4.0), S::from(5.0) ];
/// let nv_1   = NumVec::new(v_1);
/// let av     = ad_from_vector( vec![nv_0, nv_1] );
/// let v      = ad_to_vector(av);
/// assert_eq!( v[0].get(0), S::from(2.0) );
/// assert_eq!( v[0].get(1), S::from(3.0) );
/// assert_eq!( v[1].get(0), S::from(4.0) );
/// assert_eq!( v[1].get(1), S::from(5.0) );
/// ```
pub fn ad_to_vector<V> ( avec : Vec< AD<V> > ) -> Vec<V> {
    assert_ne!( avec.len() , 0 );
    let vec  = avec.into_iter().map( |ad| ad.value).collect();
    vec
}
// -------------------------------------------------------------------------
// impl_ad_from_f32
/// Convert an f32 value to an AD object with no function information;
/// i.e., constant parameter.
///
/// **See Also** : example in [ad_from_value], [ad_from_vector]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::NumVec;
/// use rustad::AzFloat;
/// type V = rustad::AzFloat<f32>;
/// let ax : AD< NumVec< AzFloat<f64> > >  = (3.0 as f32).into();
/// let x  = ax.to_value();
/// assert_eq!( x.get(0).to_inner(), 3.0 as f64);
/// ```
pub fn doc_impl_ad_from_f32() { }
//
/// Implement from f32 for `AD<V>` .
///
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD<V>` is used. The rustad package automatically executes it
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
///
/// This macro can be invoked from anywhere.
macro_rules! impl_ad_from_f32{ ($V:ty) => {
    impl From<f32> for crate::AD<$V> {
        fn from( f32_value : f32 ) -> crate::AD<$V> {
            let tape_id         = 0;
            let index           = 0;
            let ad_type         = crate::ad::ADType::ConstantP;
            let value      : $V = f32_value.into();
            crate::AD::new(tape_id, index, ad_type, value)
        }
    }
} }
pub(crate) use impl_ad_from_f32;
// -------------------------------------------------------------------------
// impl_ad_from_f64
/// Convert an f64 value to an AD object with no function information;
/// i.e., constant parameter.
///
/// Only AD objects with f64 precision are supported; e.g.,
/// `AD<f32>` is not supported.
///
/// **See Also** : example in [ad_from_value], [ad_from_vector]
///
/// # Example
/// ```
/// use rustad::AD;
/// use rustad::AzFloat;
/// use rustad::NumVec;
/// type S          = AzFloat<f64>;
/// type V          = NumVec<S>;
/// let ax : AD<V>  = (3.0 as f64).into();
/// let x           = ax.to_value();
/// assert_eq!( x.get(0), S::from(3.0) );
/// ```
pub fn doc_impl_ad_from_f64() { }
//
/// Implement from f64 for `AD<V>` .
///
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD<V>` is used. The rustad package automatically executes it
/// for the following types: `f64` , `NumVec<f64>` .
///
/// This macro can be invoked from anywhere.
macro_rules! impl_ad_from_f64{ ($V:ty) => {
    impl From<f64> for crate::AD<$V> {
        fn from( f64_value : f64 ) -> crate::AD<$V> {
            let tape_id         = 0;
            let index           = 0;
            let ad_type         = crate::ad::ADType::ConstantP;
            let value      : $V = f64_value.into();
            crate::AD::new(tape_id, index, ad_type, value)
        }
    }
} }
pub(crate) use impl_ad_from_f64;
