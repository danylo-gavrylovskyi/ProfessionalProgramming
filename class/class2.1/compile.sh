#!/bin/bash

if [ $# -lt 3 ]; then
    echo "Usage: $0 source_file.cpp -o output_binary"
    exit 1
fi

clang++ -Wall -Wextra -Wpedantic -Werror -std=c++23 "$@"
