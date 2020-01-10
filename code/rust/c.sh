#!/bin/bash

# vars
readonly f=$1

###
# main func
###
function main() {
    echo "$(cp $f tmp.rs && rustc tmp.rs && ./tmp)"
}

# exec
main
