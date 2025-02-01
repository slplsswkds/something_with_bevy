#!/bin/bash

if ! command -v toktx &> /dev/null
then
  echo "toktx could not be found. Please install KTX tools."
  exit 1
fi

if [ -z "$1" ]; then
  echo "Usage: $0 /path/to/dir/with/textures"
  exit 1
fi

INPUT_DIR="$1"
OUTPUT_DIR="${INPUT_DIR}-ktx2"

if [ ! -d "$INPUT_DIR" ]; then
  echo "Directory $INPUT_DIR does not exist."
  exit 1
fi

mkdir -p "$OUTPUT_DIR"

echo "input dir = ${INPUT_DIR}"
echo "output dir = $OUTPUT_DIR"


toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf srgb --assign_oetf srgb --zcmp 20 "${OUTPUT_DIR}/base_color.ktx2" "${INPUT_DIR}/base_color.png"
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf srgb --assign_oetf linear --zcmp 20 "${OUTPUT_DIR}/normal_opengl.ktx2" "${INPUT_DIR}/normal_opengl.png"
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 "${OUTPUT_DIR}/ao.ktx2" "${INPUT_DIR}/ao.png"
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 "${OUTPUT_DIR}/metallic_roughness.ktx2" "${INPUT_DIR}/metallic_roughness.png"

