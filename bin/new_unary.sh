#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2026 Bradley M. Bell
# -----------------------------------------------------------------------------
if [ $# != 1 ]
then
   echo 'bin/new_unary.sh name'
   echo 'where name is the name of the new unary function; e.g., sin'
   exit 1
fi
name="$1"
if [ -e "src/op/unary/$name.rs" ]
then
   echo "new_unary.sh: src/op/unary/$name.rs already exists"
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
#
# traits.rs
file='src/float/traits.rs'
cat << EOF > temp.sed
/^pub trait FUnary/! b end
N
N
s|\$|\\
    fn $name(self) -> Self::Output;|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# az_float.rs
file='src/float/az_float.rs'
cat << EOF > temp.sed
/^        [/][/] use float_unary_function/! b end
s|\$|\\
        float_unary_function!(\$B, $name);|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# num_vec.rs
file='src/float/num_vec.rs'
cat << EOF > temp.sed
/^    [/][/] use float_unary_function/! b end
s|\$|\\
    float_unary_function!($name);|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# ad/float_core.rs
file='src/ad/f_unary.rs'
cat << EOF > temp.sed
/^    [/][/] use unary_self_borrowed/! b one
s|\$|\\
    unary_self_borrowed!($name);|
b end
#
: one
/^    [/][/] use unary_self_owned/! b end
s|\$|\\
    unary_self_owned!($name);|
b end
#
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# id.rs
file='src/op/id.rs'
cat << EOF > temp.sed
/^    [/][/] Unary Operators/! b end
s|\$|\\
    /// $name\\
    ${NAME}_OP,|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# info.rs
file='src/op/info.rs'
cat << EOF > temp.sed
/^    [/][/] unary operators/! b end
s|\$|\\
    crate::op::unary::$name::set_op_info::<V>(\\&mut result);|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# mod.rs
file='src/op/unary/mod.rs'
cat << EOF > temp.sed
/^pub mod common;/! b end
s|\$|\\
pub mod $name;|
: end
EOF
git checkout $file
sed -i $file -f temp.sed
#
# $name.rs
cat << EOF > temp.sed
s|\\([": (]\\)exp_m1\\(["_ ()]\\)|\\1$name\\2|g
s|\\([": (]\\)EXP_M1\\(["_ ()]\\)|\\1$NAME\\2|g
s|EXP_M1_OP as usize|${NAME}_OP as usize|
EOF
sed -f temp.sed src/op/unary/exp_m1.rs > src/op/unary/$name.rs
#
cat << EOF
src/op/unary/$name.rs: Fix ${name}_forward_der and ${name}_reverse_der
                       Check constraints in this set_op_info function.
src/float/az_float.rs: Check implementation of fn $name(&self) -> Self
examples/float_core.rs: Add an example for $name function values.
test/unary.rs: Add a test for $name derivatives.
EOF
#
echo 'new_unary.sh: OK'
