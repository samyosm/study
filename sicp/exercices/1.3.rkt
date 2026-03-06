#lang sicp

(define (square x) (* x x))

(define (sum-of-squares a b) (+ (square a) (square b)))

(define (smallest? a b c) (and (<= a b) (<= a c)))

(define (sum-of-largest-squares a b c)
  (cond ((smallest? a b c) (sum-of-squares b c))
    ((smallest? b a c) (sum-of-squares a c))
    ((smallest? c b a) (sum-of-squares b a))))

(sum-of-largest-squares 7 6 7)

