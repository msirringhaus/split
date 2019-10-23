#!/bin/bash

function abort() {
  echo "$@"
  exit 1
}

function assert_eq() {
    [ "$1" = "$2" ] || abort "$1 is not equal to $2"
}

if [ $# -ne 1 ]; then
    abort "Usage: $0 PATH_TO_BINARY"
fi

BIN="$1"

if [ ! -f "$BIN" ]; then
    abort "$BIN does not exist!"
fi

# Test README-functions
assert_eq "$(echo "I'm................thinking" | $BIN ".")"     "I'm thinking"
assert_eq "$(echo "How did that awk-command work again?" | $BIN -c 1,2,3,5,6 -j '===')"    "How===did===that===work===again?"
assert_eq "$(echo "I was...like...thinking...like...how did you...like...come up with that?" | $BIN -c 3,4 "...like...")"    "how did you come up with that?"
assert_eq "$(echo "I'm................thinking" | $BIN -c 2 ".")"    "thinking"
assert_eq "$(echo "I'm................thinking" | $BIN -k -c 17 ".")"    "thinking"
assert_eq "$(echo "Part1..................Something
Part10.................Another
Part100................Thing
Part1000...............End" | $BIN "." -j ": " -c 1,-1)"    "Part1: Something
Part10: Another
Part100: Thing
Part1000: End"
assert_eq "$(echo "1 2 3 4 5 6" | $BIN -c 3,2,1)"    "3 2 1"

# Complement-tests
assert_eq "$(echo {1..10} | $BIN --complement -c 9,7,8)"    "1 2 3 4 5 6 10"
assert_eq "$(echo {1..10} | $BIN --complement -c 8,-3,8)"    "1 2 3 4 5 6 7 9 10"
assert_eq "$(echo {1..10} | $BIN --complement -c 9,8,-3,8,5,3)"    "1 2 4 6 7 10"
assert_eq "$(echo "I'm...thinking" | $BIN --complement -k -c 4 ".")"    "I'm  "
assert_eq "$(echo "I'm...thinking" | $BIN --complement -k -c 3,2 ".")"    "I'm thinking"
