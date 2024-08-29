cd $(dirname "$0")
cd ..

TMP_DIR="output_tmp"

rm -rf "$TMP_DIR"
mkdir "$TMP_DIR"

python ./tools/rename.py ./output "$TMP_DIR"

ffmpeg -y -framerate 4  -i "$TMP_DIR/image_%04d.png" -vf "scale=400:-1,pad=400:400:-1:-1" -sws_flags neighbor process.mp4
ffmpeg -y -framerate 4  -i "$TMP_DIR/image_%04d.png" -vf "scale=400:-1,pad=400:400:-1:-1" -sws_flags neighbor process.gif

rm -rf "$TMP_DIR"