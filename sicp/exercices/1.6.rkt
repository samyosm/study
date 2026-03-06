#lang sicp

(define (better-guess guess x)
  (/ (+ guess (/ x guess)) 2))

(define (good-enough? guess x)
  (< (abs (- x (* guess guess))) 0.001))

(define (new-if predicate 
                then-clause 
                else-clause)
  (cond (predicate then-clause)
        (else else-clause)))

(define (sqrt-iter guess x)
  (new-if (good-enough? guess x)
      guess
      (sqrt-iter (better-guess guess x) x)))

(sqrt-iter 1.0 9)

;; The program never stops, because unlike <if>, <cond> evaluates sqrt-iter even when good-enough? is false.
