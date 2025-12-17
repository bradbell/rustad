#! /usr/bin/env bash
set -e -u
# SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
# SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
# SPDX-FileContributor: 2020-25 Bradley M. Bell
# -----------------------------------------------------------------------------
if [ "$0" != "bin/check_doc.sh" ]
then
   echo "bin/check_doc.sh: must be executed from its parent directory"
   exit 1
fi
#
export RUSTDOCFLAGS='-D warnings'
cargo doc --document-private-items
#
echo 'check_doc.sh: OK'
exit 0
