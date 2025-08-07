// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! VecSet a vector of sets class
//! : [parent module](super)
//
/// This vector of sets class is specalized to work with sparsity calculations.
///
/// It has the following features:
///
/// * The elements of the sets are usize values.
/// * The sets are identified by usize identifies.
/// * Once a set is created it is not modified.
/// * A singleton set can be created from a usize value.
/// * A set can be created as the union of other sets.
///
pub struct VecSet {
    //
    /// The number of sets is equal to the length of link.
    /// If `link[id_set]`, the The i-th set is a link.
    link  : Vec<bool>  ,
    //
    /// The number of sets is also equal to the length of start.
    ///
    /// * If `link[this_set]` is true,
    /// `other_set = data[ start[this_set] ]` is the index of
    /// another set that has the same elements as this set.
    /// In addition, `other_set < this_set`, so it you follow links
    /// you must eventually some to a set that is not a link.
    ///
    /// * If `link[this_set]` is false, `start[this_set]` is the first element
    /// of the this set. If `start[this_set]` is equal to `start[this_set+1]`,
    /// this set is empty.
    start : Vec<usize> ,
    //
    /// This vector that holds all the elements, and links, for all the sets.
    data  : Vec<usize> ,
    //
    /// For `0 <= i < arg.len()`,
    /// The set, in this vector of sets, with index
    /// `id_set = arg[i]` is an operand for this union operation.
    arg : Vec<usize>,
    //
    /// Fix i, and let `id_set = arg[i]`.
    /// If `next[i] = start[id_set+1]`,
    /// there are no more elements in the id_set set.
    /// Otherwise `data[ next[i] ]`
    /// is the next element of the id_set set;
    /// `next.len() == arg.len()` .
    next : Vec<usize>,
    //
    /// If `equal[i]`, the result of the union is equal to the set `arg[i]`;
    /// `equal.len() == arg.len()` .
    equal : Vec<bool>,
    //
    /// Vector used to order the operands for this union;
    /// `order.len() == arg.len()` .
    order : Vec<usize>,
}
//
impl VecSet {
    //
    // VecSet::new
    /// Initial the vector of sets as having no sets
    pub fn new() -> Self {
        Self {
            link   : Vec::new(),
            start  : Vec::new(),
            data   : Vec::new(),
            arg    : Vec::new(),
            next   : Vec::new(),
            equal  : Vec::new(),
            order  : Vec::new(),
        }
    }
    //
    // VecSet.singleton
    /// Creae a new set with one element.
    ///
    /// ```test
    ///     target = vs.union(element)
    /// ```
    ///
    ///  vs :
    /// is this [VecSet] object.
    ///
    /// element :
    /// is the value of the element in the new set.
    ///
    /// * target :
    /// is the identifier for the new set.
    /// It is one greater that the previous identifier returned by vs.
    ///
    pub fn singleton(self : &mut Self, element : usize) -> usize
    {   // link, start, data
        let link  = &mut self.link;
        let start = &mut self.start;
        let data  = &mut self.data;
        //
        // target
        debug_assert!( link.len() == start.len() );
        let target = link.len();
        //
        // link, start, data
        link.push( false );
        start.push( data.len() );
        data.push( element );
        //
        target
    }
    //
    // VecSet.get
    /// Get one set from the vector of sets.
    ///
    /// ```test
    ///     set = vs.get(id_set)
    /// ```
    ///
    ///  vs :
    /// is this [VecSet] object.
    ///
    /// id_set :
    /// is the identifier for the set.
    ///
    /// * set :
    /// is the set corresponding to id_set as a vector.
    /// The elements are in increasing order; i.e.,
    /// if i+1 < set.len() then `set[i] < set[i+1]`.
    ///
    pub fn get(self : &Self, mut id_set : usize) -> &[usize]
    {   //
        // link, start, data
        let link  = &self.link;
        let start = &self.start;
        let data  = &self.data;
        //
        debug_assert!( start.len() == link.len() );
        debug_assert!( id_set < link.len() );
        while link[id_set] {
            debug_assert!( data[ start[id_set] ] < id_set );
            id_set = data[ start[id_set] ];
        }
        //
        let mut end = data.len();
        if id_set + 1 < start.len() {
            end = start[id_set + 1]
        }
        &data[ start[id_set] .. end ]
    }

}

// VecSet.union
impl VecSet {
//
/// Create a new set that is the union of other sets.
///
/// * Syntax :
///
/// ```text
///     target = vs.union(sets)
/// ```
///
/// * vs :
/// is this [VecSet] object.
///
/// * sets :
/// is a vector is set identifiers that specifies which sets
/// are included in the union.
///
/// * target :
/// is the identifier for the new set that is the result of the union.
/// It is one greater that the previous identifier returned by vs.
///
pub fn union(self : &mut Self, sets : &Vec<usize> ) -> usize
{   //
    // link, start, data, arg, next, equal, order
    let link  = &mut self.link;
    let start = &mut self.start;
    let data  = &mut self.data;
    let arg   = &mut self.arg;
    let next  = &mut self.next;
    let equal = &mut self.equal;
    let order = &mut self.order;
    //
    // target
    debug_assert!( link.len() == start.len() );
    let target = start.len();
    //
    // start[target], link[target]
    start.push( data.len() );
    link.push( false );
    //
    // arg, next, equal, order
    arg.resize(0, 0);
    next.resize(0, 0);
    equal.resize(0, false);
    order.resize(0, 0);
    //
    // id_set
    for i in 0 .. sets.len() {
        let mut id_set = sets[i];
        debug_assert!( id_set < target );
        //
        while link[id_set] {
            debug_assert!( start[id_set] < start[id_set + 1] );
            debug_assert!( data[ start[id_set] ] < id_set );
            id_set = data[ start[id_set] ];
        }
        debug_assert!( ! link[id_set] );
        //
        // arg, next, equal, order
        let empty = start[id_set] == start[id_set + 1];
        if ! empty {
            let mut in_arg = false;
            for j in 0 .. arg.len() {
                if id_set == arg[j] {
                    in_arg = true;
                }
            }
            if ! in_arg {
                arg.push( id_set );
                next.push( start[id_set] );
                equal.push( true );
                order.push( order.len() );
            }
        }
    }
    //
    match arg.len() {
        0 => {
            // result is empty for this union
            debug_assert!( start[target] == data.len() );
        }
        1 => {
            // link, data
            // result is equal to argument for this union
            link[target]  = true;
            data.push(  arg[0] );
            debug_assert!( start[target] + 1 == data.len() );
        }
        _ => {
            //
            // more_elements
            let mut more_elements = true;
            while more_elements {
                //
                // order
                order.sort_by_key( |&i| {
                    let id_set = arg[i];
                    //
                    let mut result = usize::MAX;
                    if next[i] < start[id_set + 1] {
                        result = data[ next[i] ];
                    }
                    result
                } );
                //
                // data
                let first_set = arg[ order[0] ];
                debug_assert!( next[ order[0] ] < start[first_set + 1] );
                let element = data[ next[ order[0] ] ];
                data.push( element );
                //
                // next, equal, more_elements
                more_elements = false;
                for i in 0 .. arg.len() {
                    if next[i] < start[ arg[i] + 1 ] {
                        if element == data[ next[i] ]  {
                            next[i] += 1;
                            if next[i] < start[ arg[i] + 1 ] {
                                more_elements = true;
                            }
                        } else {
                            equal[i]      = false;
                            more_elements = true;
                        }
                    }
                }
            } // end: while more_elements {
            //
            // i_min
            let mut i_min = arg.len();
            for i in 0 .. arg.len() {
                if equal[i] {
                    if arg[i] < arg[i_min] {
                        i_min = i;
                    }
                }
            }
            //
            // link, data
            if i_min < arg.len() {
                link[target] = true;
                data.resize(start[target], 0);
                data.push( arg[i_min] );
                //
                // The more links, the faster future computations should be.
                // Can't easily recapture space in data for this case.
                for i in 0 .. arg.len() {
                    if i != i_min && equal[i] {
                        debug_assert!( i_min < i );
                        link[ arg[i] ] = true;
                        data[ start[ arg[i] ] ] = arg[i_min];
                    }
                }
            }
        } // end _ => {
    } // end:  match arg.len() {
    target
} // end: pub fn union(self : &self, sets : &Vec<usize> )
} // end: impl VecSet{
