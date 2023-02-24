#!/bin/bash

# This script will clean up the directory of all the files that are not needed
# cd into each directory and run cargo clean
for dir in */; do
    echo "Cleaning $dir"
    cd $dir
    cargo clean
    cd ..
done