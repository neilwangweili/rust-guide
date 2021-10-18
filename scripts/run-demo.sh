#!/usr/bin/env sh

if ! [ $# -eq 2 ]; then
  echo "Usage: run-demo.sh {target_file_path} {file_name}"
  echo "target_file_path: Real file path"
  echo "file_name: The name of target file. NOT INCLUDE '.rs' ! "
  exit 1
fi

rustc $1 && ./$2 && rm -f ./$2
