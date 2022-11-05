# Prolog Programming Assignment

## Motivation
Artificial Intelligence has gotten more attention in recent years. There are more AI tools available to the public than ever before: Alexa, Siri, Ericka, etc. not to mention services like Google Cloud Machine Learning, Deep Learning on AWS, IBM Cognitive, Microsoft Azure Machine Learning, etc. One of the early attempts to deal with AI was the development of “AI specific” languages. This is the case of Prolog, a declarative language that stores facts and rules (and other constructs as well), and has an interface where the user can query and make questions that Prolog’s engine will answer inferring from the facts and rules that where stored.
Another motivation for this assignment is to expose students to a completely different paradigm of programming. Up to this moment, most of the students have been working and coding with imperative and object-oriented languages, and a declarative language should pose a new challenge, as it requires a different way to think.

## Description
Write a Prolog program that helps to ask the questions whether this figure is contained or intersected by this other figure, where the figures can be squares and circles. The geometric constructs in your program will be:
- 2D Point
- Circle
- Square

The program must be able to answer the following questions:

English | Prolog
------- | ------
Is the circle defined by the center (a,b) with radius r contained in the square defined by its upper left corner (c,d) with side length l | `contained(circle(point2d(a,b), r), square(point2d(c,d), l))`
Is the circle defined by the center (a,b) with radius r contained in the circle defined by the center (c,d) with radius q | `contained(circle(point2d(a,b), r), circle(point2d(c,d), q))`
Is the square defined by the its upper left corner (a,b) with side length l contained in the square defined by its upper left corner (c,d) with side length t | `contained(square(point2d(a,b), l), square(point2d(c,d), t))`
Is the square defined by the its upper left corner (a,b) with side length l contained in the circle defined by the center (c,d) with radius r | `contained(square(point2d(a,b), l), circle(point2d(c,d), r))`
Does the circle defined by the center (a,b) with radius r intersect with the square defined by its upper left corner (c,d) with side length l | `intersects(circle(point2d(a,b), r), square(point2d(c,d), l))`
Does the circle defined by the center (a,b) with radius r intersect with the circle defined by the center (c,d) with radius q | `intersects(circle(point2d(a,b), r), circle(point2d(c,d), q))`
Does the square defined by the its upper left corner (a,b) with side length l intersect with the square defined by its upper left corner (c,d) with side length t | `intersects(square(point2d(a,b), l), square(point2d(c,d), t))`
Does the square defined by the its upper left corner (a,b) with side length l intersect with the circle defined by the center (c,d) with radius r | `intersects(square(point2d(a,b), l), circle(point2d(c,d), r))`




## Assignment Requirements
-	Good programming practices
- Meaningful atoms and rules naming
-	This is a strictly individual assignment
-	A query file will be provided for you to test your program, this test (and additional ones) will be used for grading.
-	Use the rules names given in the table above.

## Delivery Method
You will push your final version to the repository of the assignment before the deadline, your program must be in a file named `square-circle.pl`  **[You MUST name your program this, failure will result in zero grade]**

## Assessment and Grading
Assessment will consider the following factors in the grading of the project:
-	Adherence to instructions
-	Correct function of the program
-	No runtime errors and no warnings
-	Late deliveries will have a zero mark
-	Plagiarism will have a double zero mark (in addition to losing 10% of your final grade, you will lose an additional 10% of their final grade), besides there will be a report filed in the students’ academic record. Make sure to read the ECS Department Academic Integrity Guidelines in the Course Syllabus
-	Each program will be loaded in Prolog and tested to check if the functions are in working order.
-	The programs will be “automatically” run using a bash shell script, it is important that you follow the instructions, so the script runs smoothly.

## Extra Challenge
- Use Prolog more advance features in this project: Lists, IO and Structures. You will need to send the test queries to try your code while grading. (up to 3 extra points)
- Use points in 3D. You will need to send the test queries to try your code while grading. (up to 10 extra points)
- To take advantage of the extra credit, you must comply with the given tests first, and those tests must work!

## Instructions for Testing
A file named `test.pl` is provided. You will need to append this file to the end of your code, create a new file called `full.pl` that contains your code and then the code from `test.pl`, and then run Prolog from the command line:  

`swipl -q -f full.pl -t main > your-output.txt`

The command tells prolog to run quietly using the file `full.pl` and to execute target `main`. The query main will then run each of the queries. Once the process is finished, there will be a file named `your-output.txt` you can then compare this file with another file that is provided: `output.txt` to check if your results are correct. 
> Before turning in your assignment make sure that your file `square-circle.pl` does not have test code.

### Steps
1. Copy your `square-circle.pl` file into a new empty file called `full.pl`
2. Append the file `test.pl` at the end of your code of `full.pl`
   1. Optionally, if you are comfortable using command line: `cat square-circle.pl test.pl > full.pl`. This will concatenate your file with the test file and create a new file called `full.pl`.
3. Run the following command:
    `swipl -q -f full.pl -t main > your-output.txt`
4. Compare the file `your-ouput.txt` with the provided `output.txt`, you may use `diff` or `fc` to compare the files.
5. If your output matches, then you are ready to push the final version of your work
6. Push the repository  

### Auto Grading your Program
This repository comes with a file `grade-prolog.cpp` this file is a C++ program that opens your output file and the provided key output, and it will compare your answers with the correct answers for each of the queries. To run the program:
```
prompt$ g++ -std=c++14 grade-prolog.cpp -o grade
prompt$ ./grade output.txt your-output.txt
```

The result of running the program should be:
```
Correct Answers:  25
Queries executed: 25
Total tests:      25
Percentage of tests passed: 100
```

This program will tell you the percentage you will get from the unit tests. Most of your grade will come from that percentage. Your code will be reviewed for the assessment and grading criteria.

## Recommendation

### Windows and Mac ...
It is recommended that you use a Docker container for your assignment, this will guarantee that you have the same version of Prolog than the one you will be graded with. If you decide to use the native Windows or Mac Prolog, make sure that your program can be run from command line.

### GUI Versions
There are GUI versions for the Prolog Interpreter, but they might not help you with the assignment goals. Use them at your own risk.

## About the Parser

Steps to integrate the parser made in Rust:
1. Create a square-circle Language "program" call it `test.sc`, define some points, circles and squares, and use the contained or intersects functions. For example: 
```
definitions:
  a = point(5, 5);
  b = point(4, 7);
  c = point(8, 6);
  alpha = circle(a, 4);
  beta = circle(b, 1.5);
  gamma = square(c, 2)
operations:
  contained(beta, alpha);
  intersects(gamma, alpha)
end.
```
> NOTE: make sure you don't have any `print` statements in your operations since you are not required to create code for print in your Prolog assignment.
1. Run your parser: `cargo run test.sc -p > mytest.pl`. This will generate the Prolog code that represents your `test.sc` program.
1. `mytest.pl` should look like:
```
/* processing input file input.sc
   Lexical and Syntax analysis passed */
query(contained(circle(point2d(4,7),1.5), circle(point2d(5,5),4))).
query(intersects(square(point2d(8,6),2), circle(point2d(5,5),4))).
writeln(T) :- write(T), nl.
main:- forall(query(Q), Q -> (writeln('yes')) ; (writeln('no'))),
	halt.
```
1. Concatenate your program with the program created by Rust: `cat square-circle.pl mytest.pl > full.pl`
1. Run your Prolog program to check the test made with your sc program in `test.sc`: `swipl -q -f full.pl -t main`. Notice how the command is telling Prolog to load the concatenated file `full.pl`. Running that program with your Prolog program should have an input like:
```
yes
yes
```



