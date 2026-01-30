// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines sparsity utilities
//!
//! Link to [parent module](super)
// ----------------------------------------------------------------------------
// use
use crate::{
    SparsityPattern,
};
// ----------------------------------------------------------------------------
// coloring
/// Compute a coloring that can be used for a sparse derivative calculation
/// of a subset of a Jacobian.
///
/// ```text
///     color = coloring(m, n, pattern, sub_pattern)
/// ```
///
/// * m :
///   number of rows in the matrix the sparsity pattern corresponds to.
///
/// * n :
///   number of columns in the matrix the sparsity pattern corresponds to.
///
/// * pattern :
///   If (i, j) is in *pattern* then, i < m, j < n, and the (i,j) entry
///   in the Jacobian may be non-zero.
///
/// * sub_pattern :
///   is a subset of *pattern* that we wish to calculate.
///
/// * color :
///   This is a coloring for the subset of the Jacobain. 
///   If color[j] == n, (i, j) does not appear in *sub_pattern* for any i.
///   Otherwise, color[j] < n. 
///
///   Suppose j1 != j2, color[j1] == color[j2], (i, j1) is in *pattern*,
///   (i, j2) is in *pattern*. It follows that neither (i, j1) or (i, j2)
///   is in *sub_pattern* .
///
///   Given the conditions above, this routine tries to minimuze the number 
///   of different colors used in color. In addition, the colors are sequential
///   starting at zero, (except for the special color n mentioned above).
///
/// * Forward Mode :
///   Suppose *pattern* is a sparsity pattern for the Jacobian of an [ADfn] 
///   object f. Fix a color k and suppose that the
///   [forward derivative](doc_forward_der) is calculated,
///   for this function object, with the domain derivative one (zero) 
///   for each domain component j that has color[j] = k (color[j] != k).
///   For each j with color[j] = k and each i with (i, j) is *sub_pattern*, 
///   the forward mode range component i is the (i, j) component of the 
///   Jacobain of f.
///
/// * Reverse Mode :
///   Suppose *pattern* is a sparsity pattern for the transpose of the
///   Jacobian of an [ADfn] object f. Fix a color k and suppose that the
///   [reverse derivative](doc_reverse_der) is calculated,
///   for this function object, with the range derivative one (zero) 
///   for each range component i that has color[i] = k (color[i] != k).
///   For each i with color[i] = k and each j with (j, i) is *sub_pattern*, 
///   the reverse mode domain component j is the (i, j) component of the 
///   Jacobain of f.
///
/// * Reference :
///   See GreedyPartialD2Coloring Algorithm Section 3.6.2 of
///   Graph Coloring in Optimization Revisited by
///   Assefaw Gebremedhin, Fredrik Maane, Alex Pothen
///
///   The algorithm was modified (by Brad Bell) to take advantage when
///   only a subset of the sparsity pattern needs to be calculated.
///
pub fn coloring(
    m           : usize            ,
    n           : usize            ,
    pattern     : &SparsityPattern ,
    sub_pattern : &SparsityPattern ,
) -> Vec<usize> {
    //
    // col_major
    // column major order for pattern
    let mut col_major : Vec<usize> = (0 .. pattern.len()).collect();
    col_major.sort_by_key( |&ell| { let [i,j] = pattern[ell]; [j, i] } );
    //
    // row_major
    // row major order for sub_pattern
    let mut row_major : Vec<usize> = (0 .. sub_pattern.len()).collect();
    row_major.sort_by_key( |&ell| pattern[ell] );
    //
    // col_begin
    let mut col_begin = vec![0; n+1];
    let mut j         = 0;
    for (ell, p) in col_major.iter().enumerate() {
        let [_i1, j1] = pattern[*p];
        if j < j1 {
            col_begin[j+1] = ell;
            j += 1;
        }
    }
    for begin_j in col_begin[j+1 .. n+1 ].iter_mut() {
        *begin_j = pattern.len();
    }
    //
    // row_begin
    let mut row_begin = vec![0; m+1];
    let mut i         = 0;
    for (ell, p) in row_major.iter().enumerate() {
        let [i1, _j1] = sub_pattern[*p];
        if i < i1 {
            row_begin[i+1] = ell;
            i += 1;
        }
    }
    for begin_i in row_begin[i+1 .. m+1].iter_mut() {
        *begin_i = sub_pattern.len();
    }
    //
    // col_in_sub_pattern
    let mut col_in_sub_pattern = vec![false; n];
    for [_i, j] in sub_pattern {
        col_in_sub_pattern[*j] = true;
    }
    //
    // color
    let mut color : Vec<usize> = Vec::with_capacity(n);
    let mut k = 0;
    for flag in col_in_sub_pattern.iter() {
        if *flag {
            color.push( k );
            k += 1;
        } else {
            color.push( n );
        }
    }
    //
    // forbidden
    let mut forbidden = vec![true; n];
    //
    // n_color
    let mut n_color = 0;
    //
    // color[j]
    // determine the final color for index j
    for j in 0 .. n { if color[j] < n {
        //
        // forbidden
        for forbidden_k in forbidden[0 .. n_color].iter_mut() {
            *forbidden_k = false;
        }
        //
        // ell 
        let begin_j = col_begin[j];
        let end_j   = col_begin[j+1];
        for ell in &col_major[begin_j .. end_j] {
            //
            // i
            let [i, _j1] = pattern[*ell];
            debug_assert!( pattern[*ell][1] == j );
            //
            // p
            let begin_i = row_begin[i];
            let end_i   = row_begin[i+1];
            for p in &row_major[begin_i .. end_i] {
                //
                // j1
                let [_i, j1] = sub_pattern[*p];
                debug_assert!( sub_pattern[*p][0] == i );
                //
                // forbidden
                if j1 < j && color[j1] < n {
                    forbidden[ color[j1] ] = true;
                }
            }
        }
        // color[j]
        let mut k = 0;
        while k < n_color && forbidden[k] {
            k += 1;
        }
        color[j] = k;
        //
        // n_color
        if k == n_color {
            n_color += 1;
        }
    } }
    color
}
