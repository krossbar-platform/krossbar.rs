#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
DESTPATH="${SCRIPTPATH}/book"
BOOKPATH="${SCRIPTPATH}/../krossbar-book"

mkdir -p "${DESTPATH}"
mdbook build --dest-dir "${DESTPATH}" "${BOOKPATH}"

echo "Copying images"
cp -r "${BOOKPATH}/images" "${DESTPATH}" 
