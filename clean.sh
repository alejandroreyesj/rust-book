#!/bin/bash

# cd into each cargo project and run cargo clean.
for dir in */; do
    echo "Cleaning $dir"
    cd $dir
    cargo clean
    cd ..
done