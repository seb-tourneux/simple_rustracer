#!/bin/bash

cd $(dirname "$0")
cd ..

set -x

TMP_DIR="output_tmp"

rm -rf "$TMP_DIR"
mkdir "$TMP_DIR"

python ./tools/rename.py ./output "$TMP_DIR"

FFMPEG_COMMAND="ffmpeg -y -framerate 8  -i \"$TMP_DIR/image_%04d.png\" -vf \"scale=800:-1,pad=800:800:-1:-1, tpad=stop_mode=clone:stop_duration=2\" -sws_flags neighbor"

echo "FFMPEG_COMMAND $FFMPEG_COMMAND"

eval $FFMPEG_COMMAND process.mp4
eval $FFMPEG_COMMAND process.gif

rm -rf "$TMP_DIR"