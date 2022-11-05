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