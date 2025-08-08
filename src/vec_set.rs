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
/// Note that there is no way to create an empty set.
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
    /// of the this set. None of the sets are empty; i.e.,
    ///  `start[this_set] < start[this_set+1]` .
    start : Vec<usize> ,
    //
    /// This vector that holds all the elements, and links, for all the sets.
    data  : Vec<usize> ,
    //
    /// For `0 <= i < arg.len()`,
    /// The set, in this vector of sets, with index
    /// `id_set = arg[i]` is an operand for this union operation.
    /// In addition, the set is not a link.
    arg : Vec<usize>,
    //
    /// Fix i, and let `id_set = arg[i]`.
    /// If `next[i] = start[id_set + 1]` or `next[i] == data.len()` ,
    /// there are no more elements in the `id_set` set.
    /// Otherwise `data[ next[i] ]`
    /// is the next element of the id_set set;
    /// `next.len() == arg.len()` .
    next : Vec<usize>,
    //
    /// If `equal[i]`, the result of the union is equal to the set `arg[i]`;
    /// `equal.len() == arg.len()` .
    equal : Vec<bool>,
    //
    /// This vector is used to order the operands for this union;
    /// `order.len() == arg.len()` .
    order : Vec<usize>,
}
//
impl VecSet {
    //
    // VecSet::new
    // Initial the vector of sets as having no sets.
    // The first set identifier after this operation will be zero.
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
    /// Create a new set with one element.
    ///
    /// ```test
    ///     target = vs.singleton(element)
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
    /// # Example
    /// ```
    /// let mut vs  = rustad::vec_set::VecSet::new();
    /// let element = 3usize;
    /// let target  = vs.singleton(element);
    /// let set     = vs.get(target);
    /// assert_eq!( target,    0 );
    /// assert_eq!( set.len(), 1 );
    /// assert_eq!( set[0],    3 );
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
    /// if `i+1 < set.len()` then `set[i] < set[i+1]`.
    ///
    /// # Example
    /// ```
    /// let mut vs   = rustad::vec_set::VecSet::new();
    /// let id_2     = vs.singleton(2);
    /// let id_3     = vs.singleton(3);
    /// let set      = vs.get( id_3 );
    /// assert_eq!( set.len(), 1 );
    /// assert_eq!( set[0],    3 );
    /// let set      = vs.get( id_2 );
    /// assert_eq!( set.len(), 1 );
    /// assert_eq!( set[0],    2 );
    /// ```
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
    //
    // VecSet.n_data
    /// Return the number elements and links used to represent all the sets.
    ///
    /// ```test
    ///     n_data = vs.n_data(element)
    /// ```
    ///
    ///  vs :
    /// is this [VecSet] object.
    ///
    /// n_data :
    /// is the number of set elements, and link,
    /// used to represent all the sets.
    ///
    /// # Example
    /// ```
    /// let mut vs  = rustad::vec_set::VecSet::new();
    /// let id_2    = vs.singleton(2);
    /// let id_3    = vs.singleton(3);
    /// assert_eq!( 2, vs.n_element() );
    /// //
    /// // number of elements in {2}, {3}, {2,3}
    /// let sub_set = vec![ id_2, id_3 ];
    /// let id_2_3  = vs.union( &sub_set );
    /// assert_eq!( 4, vs.n_element() );
    /// //
    /// // The sets {2, 3} union {3} = {2, 3},
    /// // so it requires 4 elements plus one link to represent:
    /// // {2}, {3}, {2, 3}, and {2, 3} union {3} .
    /// let sub_set = vec![ id_2_3, id_3 ];
    /// let id_next = vs.union( &sub_set );
    /// assert_eq!( id_next, id_2_3 + 1);
    /// assert_eq!( 5, vs.n_element() );
    /// ```
    ///
    pub fn n_element(self : &Self) -> usize
    {   self.data.len() }

}

// VecSet.union
impl VecSet {
//
/// Create a new set that is the union of other sets.
///
/// * Syntax :
///
/// ```text
///     target = vs.union(sub_sets)
/// ```
///
/// * vs :
/// is this [VecSet] object.
///
/// * sub_sets :
/// is a vector is set identifiers that specifies which sets
/// are included in the union.
///
/// * target :
/// is the identifier for the new set that is the result of the union.
/// It is one greater that the previous identifier returned by vs.
///
/// # Example
/// ```
/// let mut vs   = rustad::vec_set::VecSet::new();
/// let id_2     = vs.singleton(2);
/// let id_3     = vs.singleton(3);
/// let sub_sets = vec![ id_2, id_3 ];
/// let id_union = vs.union(&sub_sets);
/// let set      = vs.get( id_union );
/// assert_eq!( set.len(), 2 );
/// assert_eq!( set[0],    2 );
/// assert_eq!( set[1],    3 );
/// ```
///
pub fn union(self : &mut Self, sub_sets : &Vec<usize> ) -> usize
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
    for i in 0 .. sub_sets.len() {
        let mut id_set = sub_sets[i];
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
        debug_assert!( start[id_set] < start[id_set + 1] );
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
    //
    debug_assert!( 0 < arg.len() );
    match arg.len() {
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
                    } else {
                        equal[i] = false;
                    }
                }
            } // end: while more_elements {
            //
            // i_min
            let mut i_min = arg.len();
            for i in 0 .. arg.len() {
                if equal[i] {
                    if i_min == arg.len() {
                        i_min = i;
                    } else {
                        if arg[i] < arg[i_min] {
                            i_min = i;
                        }
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
} // end: pub fn union(self : &self, sub_sets : &Vec<usize> )
} // end: impl VecSet{
