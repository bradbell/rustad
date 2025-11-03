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
if [ "$0" != "bin/check_test.sh" ]
then
   echo "bin/check_test.sh: must be executed from its parent directory"
   exit 1
fi
cat << EOF > temp.sed
s|^ *\\(Finished.*\\)|\\1\\n|
/Running/! b end
: loop
N
/\\ntest result:/! b loop
/running 0 tests/d
#
s| *Running unittests ||
s| *Running ||
s|; 0 measured.*||
s|\\nrunning [0-9]* test[s]*||
s|\\n\\n|\\n|g
#
: end
EOF
#
if ! cargo test >& temp.out
then
   cat temp.out
   echo 'cargo test: Error'
   exit 1
fi
sed -f temp.sed temp.out
echo 'check_test.sh Ok'
exit 0
