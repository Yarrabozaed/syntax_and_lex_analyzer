# Scheme Programming Assignment
## Motivation
Around 1930’s one mathematical model of computation was created called the Lambda Calculus, with time it was proven to be equivalent to the Turing Machine model, meaning that both models have the same computational capability. Functional programming is an elaboration based on Lambda Calculus, where programming is declarative as opposed to imperative as most programming languages students have been exposed so far. At a point in the development of Computer Science, functional programming was thought to be applicable to solve AI problems. Besides, being an academic type of programming, industry also uses functional programming, for instance [Erlang](https://www.erlang.org/) is used in the telecommunications industry. There are modern functional programming languages like [Scala](https://www.scala-lang.org/) and [F#](https://fsharp.org/). Some imperative languages also include functional capabilities, some of those are [Java](https://docs.oracle.com/javase/tutorial/java/javaOO/lambdaexpressions.html) and JavaScript.

In this assignment, students will develop a simple program in Scheme in order to practice this language and expose them to a completely different paradigm of programming. Up to this moment, most of the students have been working and coding with imperative and object-oriented languages, and a declarative language should pose a new challenge, as it requires a different way to think.

## Description
Write a Scheme program that implements the following functions:
- `(makepoint x-cor y-cor)`. This function needs to create a “list” that will have two elements: `x-cor` and `y-cor`. You will need to use the `cons` function. Additionally, it is suggested that you make a `(get-x point)` and a `(get-y point)` functions, using the `car` and `cdr` functions to retrieve the x and y values of a point.
- `(print-circle center radius)`. This function will print the properties of the circle represented by the specified center and radius. For a circle with radius 3, it will display:
```
Circle
Area:      28.27433
Perimeter: 18.84956
```
- `(print-square corner length)`. This function will print the properties of the circle represented by the specified corner and length. Note that the given corner represents the upper left corner, and that **all** squares are parallel to the x and y axis. For a square with side length 4, it will display:
```
Square
Area:      16
Perimeter: 16
```
- `(contained-circle-circle center1 radius1 center2 radius2)`. It checks whether the circle represented by `center1` and `radius1` is contained in circle represented by `center2` and `radius2`. It returns true if it is contained, false otherwise.
- `(contained-circle-square center radius corner length)`. It checks whether the circle represented by `center` and `radius` is contained in square represented by `corner` and `length`. It returns true if it is contained, false otherwise.
- `(contained-square-circle corner length center radius)`. It checks whether the square represented by `corner` and `length` is contained in circle represented by `center` and `radius`. It returns true if it is contained, false otherwise.
- `(contained-square-square corner1 length1 corner2 length2)`. It checks whether the square represented by `corner1` and `length1` is contained in square represented by `corner2` and `length2`. It returns true if it is contained, false otherwise.
- `(intersects-circle-circle center1 radius1 center2 radius2)`. It checks whether the circle represented by `center1` and `radius1` intersects the circle represented by `center2` and `radius2`. It returns true if they intersect, false otherwise.
- `(intersects-circle-square center radius corner length)`. It checks whether the circle represented by `center` and `radius`  intersects the square represented by `corner` and `length`. It returns true if they intersect, false otherwise.
- `(intersects-square-circle corner length center radius)`. It checks whether the square represented by `corner` and `length`  intersects the circle represented by `center` and `radius`. It returns true if they intersect, false otherwise.
- `(intersects-square-square corner1 length1 corner2 length2)`. It checks whether the square represented by `corner1` and `length1`  intersects the square represented by `corner2` and `length2`. It returns true if they intersect, false otherwise.
  
## Assignment Requirements
- Good programming practices
- Correct and readable indentation (IMPORTANT!)
- This is a strictly individual assignment

## Delivery Method
You will push your final version to the repository of the assignment before the deadline, your program must be in a file named `square-circle.scm`  **[You MUST name your program this, failure will result in zero grade]**

## Assessment and Grading
Assessment will consider the following factors in the grading of the project:
-	Adherence to instructions
-	Correct function of the program
-	No runtime errors
-	Late deliveries will have a zero mark
-	Plagiarism will have a double zero mark (in addition to losing 10% of your final grade, the group that plagiarizes will lose an additional 10% of their final grade), besides there will be a report filed in the students’ academic record.
-	Each program will be loaded in Scheme and tested to check if the functions are in working order.

## Testing your Program
You can load your program into scheme `scheme --load square-circle.scm` and test each of the functions. This is shown below.

```
scheme-rocks# scheme --load square-circle.scm
MIT/GNU Scheme running under GNU/Linux
Type `^C' (control-C) followed by `H' to obtain information about interrupts.

Copyright (C) 2019 Massachusetts Institute of Technology
This is free software; see the source for copying conditions. There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.

Image saved on Thursday September 5, 2019 at 11:51:46 AM
  Release 10.1.10 || Microcode 15.3 || Runtime 15.7 || SF 4.41 || LIAR/x86-64 4.118
;Loading "square-circle.scm"... done

1 ]=> (makepoint 2 3)

;Value: (2 . 3)

1 ]=> (get-x (makepoint 2 1))

;Value: 2

1 ]=> (get-y (makepoint 2 1))

;Value: 1

1 ]=> (print-circle (makepoint 0 0) 3)
Circle
Area:      28.27433
Perimeter: 18.84956
;Unspecified return value

1 ]=> (print-square (makepoint 4 3) 3)
Square
Area:      9
Perimeter: 12
;Unspecified return value

1 ]=> (contained-circle-square (makepoint 5 5) 4 (makepoint 6 4) 1)

;Value: #f

1 ]=> (contained-circle-circle (makepoint 4 7) 2 (makepoint 5 -1) 3)

;Value: #f

1 ]=> (contained-square-circle (makepoint 6 4) 1 (makepoint 5 5) 4)

;Value: #t

1 ]=> (contained-square-square (makepoint 8 6) 2 (makepoint 1 10) 7)

;Value: #f

1 ]=> (intersects-circle-square (makepoint 5 5) 4 (makepoint 8 6) 2)

;Value: #t

1 ]=> (intersects-circle-circle (makepoint 5 5) 4 (makepoint 5 -1) 3)

;Value: #t

1 ]=> (intersects-square-circle (makepoint 8 6) 2 (makepoint 5 5) 4)

;Value: #t

1 ]=> (intersects-square-square (makepoint 6 4) 1 (makepoint -1 12) 3)

;Value: #f

1 ]=> (exit)

Pulvis et umbra sumus.
scheme-rocks#

```

## Instructions for Using the Provided Test File
A file named `test.scm` is provided. You will need to append this file to your code. To this end you can run Scheme from the command line:  

`scheme --load square-circle.scm test.scm > your-output.txt`

***You may need to press the y key to indicate that you want to quit***.

The command tells Scheme to run the two files, one containing your code, and the other containing the test cases. Once the process is finished, there will be a file named `your-output.txt` you can then compare this file with another file that is provided: `output.txt` to check if your results are correct. Before turning in your assignment make sure that your file `square-circle.scm` does not have test code.

### Alternative Steps
1. Copy your `square-circle.scm` file into a new empty file called `full.scm`
1. Append the file `test.scm` at the end of your code of `full.scm`
   1. Optionally, if you are comfortable using command line: `cat square-circle.scm test.scm > full.scm`. This will concatenate your file with the test file and create a new file called `full.scm`.
1. Run the following command:
    `scheme --load full.scm > your-output.txt`
1. Compare the file `your-ouput.txt` with the provided `output.txt`, you may use `diff` or `fc` to compare the files, but you may need to do a visual inspection to compare.
1. If your output matches, then you are ready to push the final version of your work
1. Push the repository  

## Recommendation

### Mac & Windows Users
Install MIT Scheme on a Docker Ubuntu container:
  - `sudo apt-get update`
  - `sudo apt-get install mit-scheme`

### GUI Versions
There are GUI (and web) versions for the Scheme Interpreter, but they might not help you with the assignment goals. *Use them at your own risk*.

## About the Parser
*If* you want to test your Rust parser with your Scheme program, the you can follow the next steps.

Steps to integrate the parser made in Rust:
1. Create a square-circle Language "program" call it `test.sc`, define some points, circles and squares, and use the print, contained or intersects functions. For example: 
```
definitions:
  a = point(5, 5);
  b = point(4, 7);
  c = point(8, 6);
  alpha = circle(a, 4);
  beta = circle(b, 2);
  gamma = square(c, 2)
operations:
  print(alpha);
  print(beta);
  print(gamma);
  contained(beta, alpha);
  intersects(gamma, alpha)
end.
```
1. Run your parser: `cargo run test.sc -s > mytest.scm`. This will generate the Scheme code that represents your `test.sc` program.
1. Run your Scheme program to check the test made with your sc program in `test.sc`: `scheme --load square-circle.scm mytest.scm`. Notice how the command is telling scheme to load first your `square-circle.scm` and then your recently created by the parser `mytest.scm`. Running that program with your Scheme program should have an input like:
```
Circle
Area:      50.26548
Perimeter: 25.13274

Circle
Area:      12.56637
Perimeter: 12.56637

Square
Area:      4
Perimeter: 8

#t
#t
```

## Grading

The grading is done in phases:
1. A test file is going to be appended to your code to check each of the requested functions. There will be no partial credit if the program doesn't run. Each of those tests will have partial credits, see below for details. The program should not crash, if it does, there will be a deduction of up to 20 points. 
2. Your code is going to be reviewed to check for good programming practices. For **each** programming practice infraction you may lose up to 5 points.
3. Your code is going to be processed to check originality. If your code is not original, then the rules of academic integrity will be applied.

Function | Partial Credit
--------|-----------
`(makepoint x-cor y-cor)` | 4
`print-circle` | 8
`print-square` | 8
`contained-circle-circle` | 10
`contained-circle-square` | 10
`contained-square-circle` | 10
`contained-square-square` | 10
`intersects-circle-circle` | 10
`intersects-circle-square` | 10
`intersects-square-circle` | 10
`intersects-square-square` | 10

**Note:** Check out the sample output and test provided to see the likely test cases that your code is going to be graded on.
