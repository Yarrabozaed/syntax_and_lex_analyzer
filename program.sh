#!/bin/bash
read -t 3 -p  "Running Syntax and Lexical Analyzer (in Rust) on test0.sc..."
printf "\n"

cd rust-Yarrabozaed
cargo run test0.sc -s > scheme-Yarrabozaed/mytest.scm

cd scheme-Yarrabozaed
cat mytest.scm

read -t 3 -p  "Running Scheme Program..."
printf "\n"
scheme --load square-circle.scm mytest.scm
cd ..

read -t 3 -p  "Running Prolog Program..."
printf "\n"

cargo run test0.sc -p > prolog-Yarrabozaed/mytest.pl

cd prolog-Yarrabozaed

cat square-circle.pl mytest.pl > plogfile.pl
swipl -q -f plogfile.pl -t main

read -t 3 -p  "Goodbye..."
printf "\n"

