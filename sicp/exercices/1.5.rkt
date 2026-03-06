#lang sicp

(define (p) (p))

(define (test x y) 
  (if (= x 0) 
      0 
      y))

(test 0 (p))

;; Using an applicative model, our code never stops (as p is evaluated infinetly many times), whereas we get 0 in a normal model.
