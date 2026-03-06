#lang sicp

(define (better-guess guess x)
  (/ (+ guess (/ x guess)) 2))

(define (good-enough? guess x)
  (= (/ (better-guess guess x) guess) 1))

(define (sqrt-iter guess x)
  (if (good-enough? guess x)
      guess
      (sqrt-iter (better-guess guess x) x)))

(define (csqrt x) (sqrt-iter (/ x 2.0) x))

(define x 0.9)

(csqrt x)
(sqrt x)

