#lang sicp

(define (better-guess guess x)
  (/ (+ guess (/ x guess)) 2))

(define (good-enough? guess x)
  (< (abs (- x (* guess guess))) 0.001))

(define (sqrt-iter guess x)
  (if (good-enough? guess x)
      guess
      (sqrt-iter (better-guess guess x) x)))

(sqrt-iter 1.0 0.0005)

