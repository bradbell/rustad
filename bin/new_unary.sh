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
   echo "new_unary.sh: aborting because src/op/unary/$name.rs already exists"
   exit 1
fi
#
# NAME
NAME=$(echo $name | tr [a-z] [A-Z])
#
# core.rs
file='src/float/core.rs'
cat << EOF > temp.sed
/^    [/][/] unary functions/! b end
N
s|\$|\\
    fn $name(\\&self) -> Self;|
: end
EOF
sed -i $file -f temp.sed
#
# az_float.rs
file='src/float/az_float.rs'
cat << EOF > temp.sed
/^        [/][/] unary functions/! b end
s|\$|\\
        float_core_unary_function!(\$B, $name);
: end
EOF
sed -i $file -f temp.sed
#
# num_vec.rs
file='src/float/num_vec.rs'
cat << EOF > temp.sed
/^    [/][/] unary functions/! b end
s|\$|\\
    impl_unary_float_core!($name);|
: end
EOF
sed -i $file -f temp.sed
#
# ad/float_core.rs
file='src/ad/float_core.rs'
cat << EOF > temp.sed
/^    [/][/] unary functions/! b end
s|\$|\\
    impl_unary_float_core!($name);|
: end
EOF
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
sed -i $file -f temp.sed
#
# $name.rs
cat << EOF > temp.sed
s|\\([": (]\\)sin\\(["_ ()]\\)|\\1$name\\2|g
s|\\([": (]\\)SIN\\(["_ ()]\\)|\\1$NAME\\2|g
s|SIN_OP as usize|${NAME}_OP as usize|
EOF
sed -f temp.sed src/op/unary/sin.rs > src/op/unary/$name.rs
#
cat << EOF
src/op/unary/$name.rs: Fix ${name}_forward_der and ${name}_reverse_der
src/float/az_float.rs: Check implementation of fn $name(&self) -> Self
examples/float_core.rs: Add an example for $name function values.
test/unary.rs: Add a test for $name derivatives.
EOF
#
echo 'new_unary.sh: OK'
