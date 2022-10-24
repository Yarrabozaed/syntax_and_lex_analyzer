; Square Circle Test Cases

(newline)

(define alpha (makepoint 2 3))

(display "TEST 1: ----- make-point			(2 . 3)")
(newline)
(display alpha)
(newline)

(display "TEST 2: ----- get-x	    		2")
(newline)
(display (get-x alpha))
(newline)

(display "TEST 3: ----- get-y	    		3")
(newline)
(display (get-y alpha))
(newline)

(display "TEST 4: ----- print-circle	    Area=50.26548 Perimeter=25.13274")
(newline)
(print-circle (makepoint 0 1) 4)
(newline)

(display "TEST 5: ----- print-square	    Area=25 Perimeter=20")
(newline)
(print-square (makepoint -1 -5) 5)
(newline)

;
; Contained Tests
;
(display "TEST 6: ----- contained-circle-square 1	    #f")
(newline)
(display (contained-circle-square (makepoint 5 5) 4 (makepoint 6 4) 1))
;Value: #f
(newline)

(display "TEST 7: ----- contained-circle-square 2	    #t")
(newline)
(display (contained-circle-square (makepoint 4 7) 2 (makepoint 1 10) 7))
;Value: #t
(newline)

(display "TEST 8: ----- contained-circle-square 3	    #f")
(newline)
(display (contained-circle-square (makepoint 5 5) 4 (makepoint 1 10) 7))
;Value: #f
(newline)

(display "TEST 9: ----- contained-circle-circle 1	    #t")
(newline)
(display (contained-circle-circle (makepoint 4 7) 1.5 (makepoint 5 5) 4))
;Value: #t
(newline)

(display "TEST 10: ----- contained-circle-circle 2    #f")
(newline)
(display (contained-circle-circle (makepoint 4 7) 2 (makepoint 5 -1) 3))
;Value: #f
(newline)

(display "TEST 11: ----- contained-circle-circle 3    #f")
(newline)
(display (contained-circle-circle (makepoint 5 5) 4 (makepoint 5 -1) 3))
;Value: #f
(newline)

(display "TEST 12: ----- contained-square-circle 1    #t")
(newline)
(display (contained-square-circle (makepoint 6 4) 1 (makepoint 5 5) 4))
;Value: #t
(newline)

(display "TEST 13: ----- contained-square-circle 2    #f")
(newline)
(display (contained-square-circle (makepoint 8 6) 2 (makepoint 5 5) 4))
;Value: #f
(newline)

(display "TEST 14: ----- contained-square-circle 3    #f")
(newline)
(display (contained-square-circle (makepoint -1 12) 3 (makepoint 4 7) 2))
;Value: #f
(newline)

(display "TEST 15: ----- contained-square-square 1    #t")
(newline)
(display (contained-square-square (makepoint 6 4) 1 (makepoint 1 10) 7))
;Value: #t
(newline)

(display "TEST 16: ----- contained-square-square 2    #f")
(newline)
(display (contained-square-square (makepoint 8 6) 2 (makepoint 1 10) 7))
;Value: #f
(newline)

(display "TEST 17: ----- contained-square-square 3    #f")
(newline)
(display (contained-square-square (makepoint 8 6) 2 (makepoint -1 12) 3))
;Value: #f
(newline)


;
; Intersects Tests
;

(display "TEST 18: ----- intersects-circle-square 1    #t")
(newline)
(display (intersects-circle-square (makepoint 5 5) 4 (makepoint 1 10) 7))
;Value: #t
(newline)

(display "TEST 19: ----- intersects-circle-square 2    #t")
(newline)
(display (intersects-circle-square (makepoint 5 5) 4 (makepoint 8 6) 2))
;Value: #t
(newline)

(display "TEST 20: ----- intersects-circle-square 3   #f")
(newline)
(display (intersects-circle-square (makepoint 4 7) 2 (makepoint 6 4) 1))
;Value: #f
(newline)

(display "TEST 21: ----- intersects-circle-circle 1   #f")
(newline)
(display (intersects-circle-circle (makepoint 4 7) 2 (makepoint 5 -1) 3))
;Value: #f
(newline)

(display "TEST 22: ----- intersects-circle-circle 2   #t")
(newline)
(display (intersects-circle-circle (makepoint 5 5) 4 (makepoint 5 -1) 3))
;Value: #t
(newline)

(display "TEST 23: ----- intersects-circle-circle 3   #f")
(newline)
(display (intersects-circle-circle (makepoint 5 5) 4 (makepoint 4 7) 1.5))
;Value: #f
(newline)

(display "TEST 24: ----- intersects-square-circle 1   #t")
(newline)
(display (intersects-square-circle (makepoint 8 6) 2 (makepoint 5 5) 4))
;Value: #t
(newline)

(display "TEST 25: ----- intersects-square-circle 2   #f")
(newline)
(display (intersects-square-circle (makepoint -1 12) 3 (makepoint 4 7) 2))
;Value: #f
(newline)

(display "TEST 26: ----- intersects-square-circle 3   #f")
(newline)
(display (intersects-square-circle (makepoint 6 4) 1 (makepoint 5 5) 4))
;Value: #f
(newline)

(display "TEST 27: ----- intersects-square-square 1   #f")
(newline)
(display (intersects-square-square (makepoint 6 4) 1 (makepoint -1 12) 3))
;Value: #f
(newline)

(display "TEST 28: ----- intersects-square-square 2   #f")
(newline)
(display (intersects-square-square (makepoint 1 10) 7 (makepoint 6 8) 1))
;Value: #f
(newline)

(display "TEST 29: ----- intersects-square-square 3   #t")
(newline)
(display (intersects-square-square (makepoint -1 12) 3 (makepoint 1 10) 7))
;Value: #t
(newline)

(display "TEST 30: ----- intersects-square-square 4   #t")
(newline)
(display (intersects-square-square (makepoint 1 10) 7 (makepoint -1 12) 3))
;Value: #t
(newline)



(newline)
(newline)

(exit)
