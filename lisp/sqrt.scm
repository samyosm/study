(define (csqrt x)

  (define (sqrt-iter guess)
  (if (good-enough? guess) 
    guess (sqrt-iter (improve guess)) ))

  (define (good-enough? guess) 
    (< (abs (- (* guess guess) x) ) 0.0001 )  )

  (define (average x y) (/ (+ x y) 2))

  (define (improve guess) 
    (average guess (/ x guess)) )

  (sqrt-iter 1))

(display (csqrt 9))
(newline)

(display (sqrt 9))
(newline)

