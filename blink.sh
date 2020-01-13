#!/bin/bash
pins=(
  10
  23
  22
  24
  25
  26
)

while true; do
  for state in high low; do
    for pin in "${pins[@]}"; do
      sudo ./target/debug/rpap gpio "$pin" output $state
      sleep 0.2
    done
  done
done
