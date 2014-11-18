#!/bin/sh

export LIBTCOD_SRC_DIR="~/Storage/Dev/rust/libtcod/"
cp $LIBTCOD_SRC_DIR/*.dylib $OUT_DIR/
cp $LIBTCOD_SRC_DIR/terminal.png $OUT_DIR/../../../