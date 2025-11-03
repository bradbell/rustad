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
if [ "$0" != "bin/cargo_example.sh" ]
then
   echo "bin/cargo_example.sh: must be executed from its parent directory"
   exit 1
fi
cat << EOF > temp.sed
/^error:/d
/^Available examples:/d
EOF
if cargo run --example >& temp.out
then
   cat temp.out
   echo 'cargo run --example: Expected an error message'
   exit 1
fi
#
list=$(sed -f temp.sed temp.out)
for example in $list
do
   if cargo run --example $example >& temp.out
   then
      echo "cargo run --example $example: OK"
   else
      cat temp.out
      echo "cargom run --example $example: Error"
      exit 1
   fi
done
echo 'cargo_example.sh: OK'
exit 0
