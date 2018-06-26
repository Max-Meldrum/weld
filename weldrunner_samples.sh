#!/bin/bash

MAP_EXPR="map(v, |a:i32| a + i32(5))"
MAP_VEC="1 2 3 4"

echo -e "Running $MAP_EXPR with vector $MAP_VEC"
echo "Result: $(./target/debug/weldrunner "$MAP_EXPR" "$MAP_VEC")"


FILTER_EXPR="filter(v, |a:i32| a > 2)"
FILTER_VEC="1 2 3 4"
echo -e "Running $FILTER_EXPR with vector $FILTER_VEC"
echo "Result: $(./target/debug/weldrunner "$FILTER_EXPR" "$FILTER_VEC")"

