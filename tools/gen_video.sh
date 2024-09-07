#!/bin/bash

cd $(dirname "$0")
cd ..

set -x

TMP_DIR="output_tmp"

rm -rf "$TMP_DIR"
mkdir "$TMP_DIR"

python ./tools/rename.py ./output_second_book "$TMP_DIR"

FFMPEG_COMMAND="ffmpeg -y -framerate 8  -i \"$TMP_DIR/image_%04d.png\" -vf \"scale=800:-1,pad=800:800:-1:-1, tpad=stop_mode=clone:stop_duration=2\" -sws_flags neighbor"

echo "FFMPEG_COMMAND $FFMPEG_COMMAND"

eval $FFMPEG_COMMAND process_3.mp4

rm -rf "$TMP_DIR"

FFMPEG_COMMAND_CONCAT="ffmpeg -y -f concat -safe 0 -i video_files.txt -c:v libx264 -crf 10 -vf format=yuv420p"
eval $FFMPEG_COMMAND_CONCAT process.mp4
