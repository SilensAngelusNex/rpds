#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -e

cd $(dirname "$0")
cd "$(git rev-parse --show-toplevel)"

function new_readme() {
    filename=$(mktemp)

    cp README.md "$filename"

    sed -i '/^$/q' "$filename"

    grep --no-filename '//!' src/lib.rs \
        | sed 's,^//!\( \|\),,' \
        | grep -v '\[!\[.* documentation\](.*)\](.*/struct\..*\.html)' >> "$filename"

    echo "$filename"
}

check=false

args=$(getopt -l "check" -o "ch" -- "$@")

eval set -- "$args"

while [ $# -ge 1 ]; do
        case "$1" in
                --)
                    # No more options left.
                    shift
                    break
                    ;;
                -c|--check)
                    check=true
                    shift
                    ;;
                -h)
                    echo "usage: $0 [--check]"
                    exit 0
                    ;;
        esac

        shift
done

new_readme_filename=$(new_readme)

if $check; then
    if ! diff "$new_readme_filename" README.md > /dev/null; then
        echo "README.md is outdated.  Run $0 to update it." 2>&1
        exit 1
    fi
else
    mv "$new_readme_filename" README.md
fi

exit 0
