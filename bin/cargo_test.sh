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
#
echo_eval cargo test --all-targets 2>&1 | sed -f temp.sed
#
echo 'cargo_test.sh: OK'
exit 0
