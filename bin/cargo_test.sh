#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2025 Bradley M. Bell
# -----------------------------------------------------------------------------
# echo_eval
echo_eval() {
   echo $*
   eval $*
}
# -----------------------------------------------------------------------------
if [ "$0" != "bin/cargo_test.sh" ]
then
   echo "bin/cargo_test.sh: must be executed from its parent directory"
   exit 1
fi
cat << EOF > temp.sed
/Running/! d
: loop
N
/\\ntest result:/! b loop
s|\\n\\n|\\n|
s|\\n\\n|\\n|
s|(target/[^)]*)||
s| *Running *unittests *|\\n|
s| *Running *|\\n|
s|running [0-9]* tests*\\n||
s|; 0 measured.*||
EOF
if ! cargo test --doc
then
   echo 'cargo test --doc: Error'
   exit 1
fi
echo 'cargo test --doc: OK'
#
if ! cargo test >& temp.out
then
   cat temp.out
   echo 'cargo test: Error'
   exit 1
fi
sed -f temp.sed temp.out
echo 'cargo test: Ok'
exit 0
