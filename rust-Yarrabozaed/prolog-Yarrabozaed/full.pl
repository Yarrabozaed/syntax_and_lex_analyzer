/* 
; Author: Yarra Abozaed
; Class: CSC 3310 
; Due Date: November 13th, 2022
; Professor: Dr. Arias
*/

/* Contained Rules-> 
    check if first shape is contained in second shape 
*/

/* Contained - Circle - Circle */
contained(circle(point2d(A,B), R), circle(point2d(C,D), Q)) :- (   
    Q - R < sqrt(((A-C) * (A-C)) + ((B-D) * (B-D)))
    -> false

    ; true
).

/* Contained - Square - Square */
contained(square(point2d(A,B), L), square(point2d(C,D), T)) :- (
    A < C
    -> false
    ; (A + L) > (C + T)
    -> false

    ; (B - L) < (D - T)
    -> false
    ; B > D
    -> false

    ; true
).

/* Contained - Circle - Square */
contained(circle(point2d(A, B), R), square(point2d(C, D), L)) :- (
    R*2 >  L
    -> false

    ; A < C 
    -> false
    ; A > C + L
    -> false

    ; B < D-L
    -> false
    ; B > D
    -> false

    ; A-R < C
    -> false
    ; A+R > C+L
    -> false
    
    ; B-R < D-L
    -> false
    ; B+R > D
    -> false

    ; true
).

/* Contained - Square - Circle 
center of square x -> (A + (L/2))
center of square y -> (B + (L/2))
*/
contained(square(point2d(A,B), L), circle(point2d(C,D), R)) :- (
    R*2 < L
    -> false

    ; (A + (L/2)) < (C-R)
    -> false
    ; (A + (L/2)) > (C+R)
    -> false

    ; (B + (L/2)) < (D-R)
    -> false
    ; (B + (L/2)) > (D+R)
    -> false
    
    ; (C-R) > (A)
    -> false
    ; (C+R) < (A+L)
    -> false

    ; (D-R) > (B-L)
    -> false
    ; (D+R) < (B)
    -> false

    ; true
).


/* Intersect Rules-> 
    check if the two given shapes intersect one another
*/



/* Intersect - Circle - Square */
intersects(circle(point2d(A,B), R), square(point2d(C,D), L)) :- (
    (((D < B-R);(D =:= B-R)), (D-L < B-R))
    -> false
    ;(((D-L > B+R);(D-L =:= B+R)), (D > B+R))
    -> false

    ;(((C > A+R);(C =:= A+R)), (C+L > A+R))
    -> false
    ;(((C+L > A-R);(C+L =:= A-R)), (C < A-R))
    -> false

    ;(
        ((A-R) > (C)), 
        ((A+R) < (C+L)), 
        ((B-R) > (D-L)), 
        ((B+R) < (D))
    )
    -> false

    ;(
        ((A-R) < (C)), 
        ((A+R) > (C+L)), 
        ((B-R) < (D-L)), 
        ((B+R) > (D))
    )
    -> false

    ; true
).

/* Intersect - Circle - Circle */
intersects(circle(point2d(A,B), R), circle(point2d(C,D),Q))  :- (
    sqrt(((A-C) * (A-C)) + ((B-D) * (B-D))) < Q - R 
    -> false
    ; sqrt(((A-C) * (A-C)) + ((B-D) * (B-D))) < R - Q
    -> false
    ; sqrt(((A-C) * (A-C)) + ((B-D) * (B-D))) > R + Q 
    -> false
    ; true
).

/* Intersect - Square - Square */
intersects(square(point2d(A,B), L), square(point2d(C,D), T)) :- (
    ((A < C) , ((A + L) > (C + T)) , ((B - L) < (D - T)) , (B > D))
    -> false
    
    ;((A > C) , ((A + L) < (C + T)) , ((B - L) > (D - T)) , (B < D))
    -> false

    ; A+L < C
    -> false
    ; C+T < A
    -> false

    ; B-L > D
    -> false
    ; D-T > B
    -> false

    ;true
).


/* Intersect - Square - Circle */
intersects(square(point2d(C,D), L), circle(point2d(A,B), R)) :- (
    (((D < B-R);(D =:= B-R)), (D-L < B-R))
    -> false
    ;(((D-L > B+R);(D-L =:= B+R)), (D > B+R))
    -> false

    ;(((C > A+R);(C =:= A+R)), (C+L > A+R))
    -> false
    ;(((C+L > A-R);(C+L =:= A-R)), (C < A-R))
    -> false

    ;(
        ((A-R) > (C)), 
        ((A+R) < (C+L)), 
        ((B-R) > (D-L)), 
        ((B+R) < (D))
    )
    -> false

    ;(
        ((A-R) < (C)), 
        ((A+R) > (C+L)), 
        ((B-R) < (D-L)), 
        ((B+R) > (D))
    )
    -> false

    ; true
).

/*
    Test File for Prolog Assignment
    This test file has 25 tests for the operations:
        Contains with circles and squares
        Intersects with circles and squares
*/

query(contained(circle(point2d(5, 5), 4), square(point2d(6, 4), 1))).       /* Contained Circle - Square */
query(contained(circle(point2d(4, 7), 2), square(point2d(1, 10), 7))).
query(contained(circle(point2d(5, 5), 4), square(point2d(1, 10), 7))).
query(contained(circle(point2d(4, 7), 1.5), circle(point2d(5, 5), 4))).     /* Contained Circle - Circle */
query(contained(circle(point2d(4, 7), 2), circle(point2d(5, -1), 3))).
query(contained(circle(point2d(5, 5), 4), circle(point2d(5, -1), 3))).
query(contained(square(point2d(6, 4), 1), circle(point2d(5, 5), 4))).       /* Contained Square - Circle */
query(contained(square(point2d(8, 6), 2), circle(point2d(5, 5), 4))).
query(contained(square(point2d(-1, 12), 3), circle(point2d(4, 7), 2))).
query(contained(square(point2d(6, 4), 1), square(point2d(1, 10), 7))).      /* Contained Square - Square */
query(contained(square(point2d(8, 6), 2), square(point2d(1, 10), 7))).
query(contained(square(point2d(8, 6), 2), square(point2d(-1, 12), 3))).
query(intersects(circle(point2d(5, 5), 4), square(point2d(1, 10), 7))).     /* Intersect Circle - Square */
query(intersects(circle(point2d(5, 5), 4), square(point2d(8, 6), 2))).
query(intersects(circle(point2d(4, 7), 2), square(point2d(6, 4), 1))).
query(intersects(circle(point2d(4, 7), 2), circle(point2d(5, -1), 3))).     /* Intersect Circle - Circle */
query(intersects(circle(point2d(5, 5), 4), circle(point2d(5, -1), 3))).
query(intersects(circle(point2d(5, 5), 4), circle(point2d(4, 7), 1.5))).
query(intersects(square(point2d(8, 6), 2), circle(point2d(5, 5), 4))).      /* Intersect Square - Circle */
query(intersects(square(point2d(-1, 12), 3), circle(point2d(4, 7), 2))).
query(intersects(square(point2d(6, 4), 1), circle(point2d(5, 5), 4))).
query(intersects(square(point2d(6, 4), 1), square(point2d(-1, 12), 3))).    /* Intersect Square - Square*/
query(intersects(square(point2d(1, 10), 7), square(point2d(6, 8), 1))).
query(intersects(square(point2d(-1, 12), 3), square(point2d(1, 10), 7))).
query(intersects(square(point2d(1, 10), 7), square(point2d(-1, 12), 3))).


writeln(T) :- write(T), nl.

main:- forall(query(Q), Q -> (writeln('yes')) ; (writeln('no'))),
	halt.
