; Author: Yarra Abozaed
; Class: CSC 3310 
; Due Date: Ocotber 30th, 2022
; Professor: Dr. Arias

; Binding value of pi
(define pi 3.14159)

;Create a 'list' (a cons pair of points) with an x-cor and y-cor,
;defining the components of a point
(define (makepoint x-cor y-cor)
    (cons x-cor y-cor)
)

;return the x value of a point 
(define (get-x point)
    (car point)
)

;return the y value of a point
(define (get-y point)
    (cdr point)
)

;Print the properties of a circle, i.e. area and perimeter, 
;using center and radius
(define (print-circle center radius)
    (display "Circle")
    (newline)
    (display "Area:      ")
    (display (* pi radius radius))
    (newline)
    (display "Perimeter: ")
    (display (* pi radius 2))
    (newline)
)

;Print the properties of a square, i.e. area and perimeter, 
;using the upper left corner and length
(define (print-square corner length)
    (display "Square")
    (newline)
    (display "Area:      ")
    (display (* length length))
    (newline)
    (display "Perimeter: ")
    (display (* length 4))
    (newline)
)

;Check if circle 1 is contained in circle 2 
(define (contained-circle-circle center1 radius1 center2 radius2)
    (define x1 (get-x center1))
    (define x2 (get-x center2))
    (define y1 (get-y center1))
    (define y2 (get-y center2))

    (define d (sqrt( + (* (- x1 x2) (- x1 x2)) ( * (- y1 y2) (- y1 y2)))))

    (cond 
        ((> d (- radius2 radius1)) #f) 
        (#t)
    )
)

;Check if circle 1 and circle 2 are intersecting
(define (intersects-circle-circle center1 radius1 center2 radius2)
    (define x1 (get-x center1))
    (define x2 (get-x center2))
    (define y1 (get-y center1))
    (define y2 (get-y center2))

    (define d (sqrt( + (* (- x1 x2) (- x1 x2)) ( * (- y1 y2) (- y1 y2)))))

    (cond 
        ((> d (+ radius2 radius1)) #f) 
        ((< d (- radius2 radius1)) #f) 
        ((< d (- radius1 radius2)) #f) 
        (#t)
    )
)

;Check if sqaure 1 is contained in square 2
(define (contained-square-square corner1 length1 corner2 length2)
    (define xmin2 (get-x corner2))
    (define xmax2 (+ length2 xmin2))
    (define ymax2 (get-y corner2))
    (define ymin2 (- ymax2 length2))

    (define xmin1 (get-x corner1))
    (define xmax1 (+ length1 xmin1))
    (define ymax1 (get-y corner1))
    (define ymin1 (- ymax1 length1))

    ;the smallest possible x & y values in S2 have to be 
    ;   larger than the mins in S1
    ;but also the largest possible x & y values in S2 have to be 
    ;   smaller than the maxs in S2
    (cond 
        ( (or (< xmin1 xmin2) (> xmax1 xmax2) (< ymin1 ymin2) (> ymax1 ymax2)) #f)
        (#t)
    )
)

;Check if square 1 and square 2 are intersecting
(define (intersects-square-square corner1 length1 corner2 length2)
    (define xmin2 (get-x corner2))
    (define xmax2 (+ length2 xmin2))
    (define ymax2 (get-y corner2))
    (define ymin2 (- ymax2 length2))

    (define xmin1 (get-x corner1))
    (define xmax1 (+ length1 xmin1))
    (define ymax1 (get-y corner1))
    (define ymin1 (- ymax1 length1))

    ;checking for conditions, including if S1 contains S2 or S2 contains S1
    ;   accounts for case where both squares are identical
    (cond 
        ( (or (< xmax1 xmin2) (< xmax2 xmin1) (> ymin1 ymax2) (> ymin2 ymax1)) #f)
        ( (and (< xmin1 xmin2) (> xmax1 xmax2) (< ymin1 ymin2) (> ymax1 ymax2)) #f)
        ( (and (> xmin1 xmin2) (< xmax1 xmax2) (> ymin1 ymin2) (< ymax1 ymax2)) #f)
        (#t)
    )
)

;Checks whether the circle represented by center and radius is contained 
;   in square represented by corner and length. 
(define (contained-circle-square center radius corner length)
    (define xmin (get-x corner))
    (define xmax (+ length xmin))
    (define ymax (get-y corner))
    (define ymin (- ymax length))

    (define xcircle (get-x center))
    (define ycircle (get-y center))
    (define xminc (- xcircle radius))
    (define xmaxc (+ xcircle radius))
    (define yminc (- ycircle radius))
    (define ymaxc (+ ycircle radius))

    ;if the diameter is bigger than the length, or the center of
    ;   the circle is outside the square, result is #f
    (cond 
        ( (> (* radius 2 ) length) #f)                      ;diameter is too big
        ( (or (< xcircle xmin) (> xcircle xmax)) #f)        ;center point is not within x bounds
        ( (or (< ycircle ymin) (> ycircle ymax)) #f)        ;center point is not within y bounds
        ( (or (< xminc xmin) (> xmaxc xmax) (< yminc ymin) (> ymaxc ymax)) #f)
        (#t)
    )
)

;Checks whether the circle represented by center and radius intersects 
;   the square represented by corner and length. 
(define (intersects-circle-square center radius corner length)
    (define xmin (get-x corner))
    (define xmax (+ length xmin))
    (define ymax (get-y corner))
    (define ymin (- ymax length))

    (define xcircle (get-x center))
    (define ycircle (get-y center))
    (define xminc (- xcircle radius))
    (define xmaxc (+ xcircle radius))
    (define yminc (- ycircle radius))
    (define ymaxc (+ ycircle radius))

    (cond 
        ( (and (or (< ymax yminc) (= ymax yminc)) (< ymin yminc)) #f)       ;square is below circle
        ( (and (or (> ymin ymaxc) (= ymin ymaxc)) (> ymax ymaxc )) #f)      ;square is above circle
        ( (and (or (> xmin xmaxc) (= xmin xmaxc)) (> xmax xmaxc)) #f)       ;square is to the right of circle
        ( (and (or (< xmax xminc) (= xmax xminc)) (< xmin xminc)) #f)       ;square is to the left of circle
        ( (and (< xminc xmin) (> xmaxc xmax) (< yminc ymin) (> ymaxc ymax)) #f)     ;circle is fully contained in square with not overlap
        ( (and (> xminc xmin) (< xmaxc xmax) (> yminc ymin) (< ymaxc ymax)) #f)     ;square is fully contained in circle with no overlap
        (#t)
    )
)

;checks whether the square represented by corner and length is contained in 
;   circle represented by center and radius. 
(define (contained-square-circle corner length center radius)
    (define xmin (get-x corner))
    (define xmax (+ length xmin))
    (define ymax (get-y corner))
    (define ymin (- ymax length))

    (define xcircle (get-x center))
    (define ycircle (get-y center))
    (define xminc (- xcircle radius))
    (define xmaxc (+ xcircle radius))
    (define yminc (- ycircle radius))
    (define ymaxc (+ ycircle radius))

    (define square_centerx (+ xmin (/ length 2)))
    (define square_centery (+ ymin (/ length 2)))


    ;if the diameter is less than the length, or the center of
    ;   the square is outside the circle, result is #f
    (cond 
        ( (< (* radius 2 ) length) #f)                                      ;diameter is too small
        ( (or (< square_centerx xminc) (> square_centerx xmaxc)) #f)        ;center of square is not within x bounds
        ( (or (< square_centery yminc) (> square_centery ymaxc)) #f)        ;center of square is not within y bounds
        ( (or (> xminc xmin) (< xmaxc xmax) (> yminc ymin) (< ymaxc ymax)) #f)     
        (#t)
    )
)


;Checks whether the square represented by corner and length intersects the 
;   circle represented by center and radius. 
(define (intersects-square-circle corner length center radius)
    (define xmin (get-x corner))
    (define xmax (+ length xmin))
    (define ymax (get-y corner))
    (define ymin (- ymax length))

    (define xcircle (get-x center))
    (define ycircle (get-y center))
    (define xminc (- xcircle radius))
    (define xmaxc (+ xcircle radius))
    (define yminc (- ycircle radius))
    (define ymaxc (+ ycircle radius))

    (cond 
        ( (and (or (< ymax yminc) (= ymax yminc)) (< ymin yminc)) #f)       ;square is below circle
        ( (and (or (> ymin ymaxc) (= ymin ymaxc)) (> ymax ymaxc )) #f)      ;square is above circle
        ( (and (or (> xmin xmaxc) (= xmin xmaxc)) (> xmax xmaxc)) #f)       ;square is to the right of circle
        ( (and (or (< xmax xminc) (= xmax xminc)) (< xmin xminc)) #f)       ;square is to the left of circle
        ( (and (< xminc xmin) (> xmaxc xmax) (< yminc ymin) (> ymaxc ymax)) #f)     ;circle is fully contained in square with not overlap
        ( (and (> xminc xmin) (< xmaxc xmax) (> yminc ymin) (< ymaxc ymax)) #f)     ;square is fully contained in circle with no overlap
        (#t)
    )
)