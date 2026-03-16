#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2026 Bradley M. Bell
# -----------------------------------------------------------------------------
if [ $# != 1 ]
then
   echo 'bin/new_binary.sh name'
   echo 'where name is the name of the new binary function; e.g., sin'
   exit 1
fi
name="$1"
if [ -e "src/op/binary/$name.rs" ]
then
   echo "new_binary.sh: src/op/binary/$name.rs already exists"
   response=''
   while [ "$response" != 'replace' ] && [ "$response" != 'abort' ]
   do
      read -p "[replace, abort] ?" response
   done
   if [ "$response" == 'abort' ]
   then
      exit 1
   fi
fi
#
# NAME
NAME=$(echo $name | tr [a-z] [A-Z])
# -----------------------------------------------------------------------------
# ad/float_core.rs
file='src/ad/f_binary.rs'
cat << EOF > temp.sed
/^ *GT_OP,\$/s|\$|\\
    ${NAME}_OP,|
#
s|^\\( *\\)impl_f_binary_\\(..\\)_borrow!( num_gt, GT_OP );|&\\
\\1impl_f_binary_\2_borrow!( $name, ${NAME}_OP );|
#
s|^\\( *\\)impl_f_binary_\\(..\\)_own!( num_gt );|&\\
\\1impl_f_binary_\2_own!( $name );|
EOF
git checkout $file
sed -i $file -f temp.sed
# ----------------------------------------------------------------------------
# src/float/*.rs
# ----------------------------------------------------------------------------
# traits.rs
file='src/float/traits.rs'
cat << EOF > temp.sed
/fn num_gt(self, rhs : Rhs) -> Self::Output;/s|\$|\\
    //\\
    // $name(self, rhs)\\
    fn $name(self, rhs : Rhs) -> Self::Output;|
EOF
git checkout $file
sed -i $file -f temp.sed
# ----------------------------------------------------------------------------
# az_float.rs
file='src/float/az_float.rs'
cat << EOF > temp.sed
/impl_f_binary_function_borrow!( \$B, num_gt, >  );/s|\$|\\
        impl_f_binary_function_borrow!( \$B, $name );|
/impl_f_binary_function_own!( num_gt );/s|\$|\\
    impl_f_binary_function_own!( $name );|
EOF
git checkout $file
sed -i $file -f temp.sed
# ----------------------------------------------------------------------------
# num_vec.rs
file='src/float/num_vec.rs'
cat << EOF > temp.sed
/impl_f_binary_num_vec_borrow!( num_gt );/s|\$|\\
    impl_f_binary_num_vec_borrow!( $name );|
/impl_f_binary_num_vec_own!( num_gt );/s|\$|\\
    impl_f_binary_num_vec_own!( $name );|
EOF
git checkout $file
sed -i $file -f temp.sed
# -----------------------------------------------------------------------------
# src/op/*.rs, src/op/binary/*.rs
# -----------------------------------------------------------------------------
# id.rs
file='src/op/id.rs'
cat << EOF > temp.sed
s|^\\( *\\)GT_OP,\$|&\\n\\1/// $name(lhs, rhs)\\n\\1${NAME}_OP,|
EOF
git checkout $file
sed -i $file -f temp.sed
# ----------------------------------------------------------------------------
# binary/common.rs
file='src/op/binary/common.rs'
if [[ ${#NAME} -le '2' ]]
then
cat << EOF > temp.sed
s|^\\( *\\)id::GT_OP\\( *\\)=>.*|&\\
\\1id::${NAME}_OP\\2=> true,|
EOF
else
   let "skip = ${#NAME} - 2"
cat << EOF > temp.sed
s|^\\( *\\)id::GT_OP \\{$skip\\}\\( *\\)=>.*|&\\
\\1id::${NAME}_OP\\2=> true ,|
EOF
fi
git checkout $file
sed -i $file -f temp.sed
# -----------------------------------------------------------------------------
# info.rs
file='src/op/info.rs'
cat << EOF > temp.sed
/crate::op::binary::num_cmp::set_op_fns::<V>(&mut result);/s|\$|\\
    crate::op::binary::$name::set_op_fns::<V>(\\&mut result);|
EOF
git checkout $file
sed -i $file -f temp.sed
# -----------------------------------------------------------------------------
# mod.rs
file='src/op/binary/mod.rs'
cat << EOF > temp.sed
/pub mod num_cmp/s|\$|\\
pub mod $name;|
EOF
git checkout $file
sed -i $file -f temp.sed
# -----------------------------------------------------------------------------
# $name.rs
cat << EOF > temp.sed
s|powf|$name|g
s|POWF|${NAME}|
EOF
sed -f temp.sed src/op/binary/powf.rs > src/op/binary/$name.rs
#
cat << EOF
src/op/binary/$name.rs: Fix ${name}_forward_der and ${name}_reverse_der
                        Check constraints in this set_op_fns function.
examples/f_binary.rs: Add an example for $name function values.
tests/$name.rs: Add a test for $name derivatives.
EOF
#
echo 'new_binary.sh: OK'
