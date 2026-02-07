#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2026 Bradley M. Bell
# -----------------------------------------------------------------------------
# TODO: edit info.rs and src/op/mod.rs
#
# name
echo 'Warning, this will git reset --hard: use control-C to abort'
read -p 'Input name of new unary function: ' name
mv bin/new_unary.sh new_unary.tmp
git reset --hard
mv new_unary.tmp bin/new_unary.sh
#
# NAME
NAME=$(echo $name | tr [a-z] [A-Z])
#
# core.rs
file='src/float/core.rs'
cat << EOF > temp.sed
/^    [/][/] unary functions/! b end
s|\$|\\
    //\\
    // $name\\
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
        fn $name(\\&self) -> Self { Self( self.0.$name() ) }|
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
    crate::op::$name::set_op_info::<V>(\\&mut result);|
: end
EOF
sed -i $file -f temp.sed
#
# mod.rs
file='src/op/mod.rs'
cat << EOF > temp.sed
/[/][/] unary functions/! b end
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
sed -f temp.sed src/op/sin.rs > src/op/$name.rs
#
cat << EOF
src/op/$name.rs: Fix the functions ${name}_forward_der and ${name}_reverse_der
src/float/core.rs: Add an example for $name function values.
test/unary.rs: Add a test for $name derivatives.
EOF
#
echo 'new_unary.sh: OK'
