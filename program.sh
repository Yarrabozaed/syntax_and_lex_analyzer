#!/bin/bash
read -t 3 -p  "Running Syntax and Lexical Analyzer (in Rust) on test0.sc..."
echo "\n"

cd rust-Yarrabozaed
cargo run test0.sc -s > scheme-Yarrabozaed/mytest.scm

cd scheme-Yarrabozaed
cat mytest.scm

read -t 3 -p  "Running Scheme Program..."
echo "\n"
scheme --load square-circle.scm mytest.scm
cd ..

read -t 3 -p  "Running Prolog Program..."
echo "\n"

read -t 3 -p  "Goodbye..."
echo "\n"

