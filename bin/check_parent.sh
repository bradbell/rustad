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
if [ "$0" != "bin/check_parent.sh" ]
then
   echo "bin/check_parent.sh: must be executed from its parent directory"
   exit 1
fi
#
# sed
source bin/grep_and_sed.sh
#
# src_list
src_list=$(find src -name '*.rs' | $sed \
   -e '/^temp$/d' \
   -e '/^temp\./d' \
   -e '/\/temp\.[^/]*$/d' \
   -e '/^src\/lib.rs$/d' \
   -e '/^src\/bin\//d'
)
for file in $src_list
do
   if ! grep '^//! : \[parent module\](super) *$' $file > /dev/null
   then
      echo "Cannot find the following line in $file:"
      echo '//! : [parent module](super)'
      exit 1
   fi
done
echo "check_parent.sh OK"
exit 0
