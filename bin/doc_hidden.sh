#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2025 Bradley M. Bell
# -----------------------------------------------------------------------------
if [ "$0" != "bin/doc_hidden.sh" ]
then
   echo "bin/doc_hidden.sh: must be executed from its parent directory"
   exit 1
fi
#
# pattern
pattern='^ *#\[doc(hidden)\] *$'
list=$( git grep -l "$pattern" )
for file in $list
do
   mv $file $file.bak
   sed -e "/$pattern/d" $file.bak > $file
done
cargo doc --document-private-items
for file in $list
do
   mv $file.bak $file
done
echo 'bin/doc_hidden.sh: OK'
