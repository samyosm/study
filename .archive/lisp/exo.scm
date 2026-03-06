(define (sumsq x y) (+ (* x x) (* y y)))

(define (sum-of-largest x y z)
    (cond ((and (< x y) (< x z)) (sumsq y z))
         ((and (< y x) (< y z)) (sumsq x z))
         ((and (< z x) (< z y)) (sumsq x y))))

(define (a-plus-abs-b a b)
  ((if (> b 0) + -) a b))

(define (println expr) (display expr) (newline))

(println (a-plus-abs-b 4 (+ 14)))
