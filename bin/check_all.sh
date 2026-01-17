#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2025-26 Bradley M. Bell
# -----------------------------------------------------------------------------
# echo_eval
echo_eval() {
   echo $*
   eval $*
}
# -----------------------------------------------------------------------------
if [ "$0" != "bin/check_all.sh" ]
then
   echo "bin/check_all.sh: must be executed from its parent directory"
   exit 1
fi
if [ "$#" != 0 ] && [ "$#" != 1 ]
then
   echo 'usage: bin/check_all.sh [-skip_check_copy]'
   exit 1
fi
#
# skip_check_copy
skip_check_copy='no'
if [ $# == 1 ]
then
   if [ "$1" == '--skip_check_copy' ]
   then
      skip_check_copy='yes'
   else
      echo 'usage: bin/check_all.sh [-skip_check_copy]'
      exit 1
   fi
fi
#
export RUSTFLAGS="-D warnings"
#
# sed
source bin/grep_and_sed.sh
#
# rustad.long-types-*
if ls rustad.long-type-* >& /dev/null
then
   rm rustad.long-type-*
fi
#
# target/deps/debug
# target/deps/release
for build_type in debug release
do
   for subdir in deps examples incremental
   do
      dir="target/$build_type/$subdir"
      if ls $dir/* >& /dev/null
      then
         rm -r $dir/*
      fi
   done
done
#
# typos
if ! which typos > /dev/null
then
   echo 'Install typos using: cargo install typos-cli'
   exit 1
fi
echo_eval typos
#
if [ "$skip_check_copy" == 'no' ]
then
   bin/check_copy.sh
fi
#
# check_list
check_list=$(ls bin/check_* | $sed \
   -e '/^bin[/]check_xrst.sh/d' \
   -e '/^bin[/]check_all.sh/d' \
   -e '/^bin[/]check_copy.sh/d' \
)
for check in $check_list
do
   echo_eval $check
done
#
# src/bin/normsq.rs
echo_eval cargo run --release --bin normsq
#
# src/bin/ad_fn.rs
echo_eval cargo run --release --bin ad_fn
echo
#
echo "check_all.sh OK"
exit 0
