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
if [ "$0" != "bin/check_all.sh" ]
then
   echo "bin/check_all.sh: must be executed from its parent directory"
   exit 1
fi
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
   dir="target/$build_type/deps"
   if [ -e $dir ]
   then
      rm $dir/*
   fi
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
# sed
source bin/grep_and_sed.sh
#
# check_list
check_list=$(ls bin/check_* | $sed \
   -e '/^bin[/]check_xrst.sh/d' \
   -e '/^bin[/]check_all.sh/d' \
)
for check in $check_list
do
   echo_eval $check
done
#
# cargo doc
echo
echo_eval cargo doc --document-private-items
echo
echo_eval cargo run --release --bin normsq
echo_eval cargo run --release --bin ad_fn
echo
echo_eval bin/cargo_test.sh
echo
#
echo "check_all.sh OK"
exit 0
