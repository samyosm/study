function csqrt(x)
  function better_guess(guess)
    return (guess + x/guess) / 2
  end

  function sqrt_iter(guess)
    bg = better_guess(guess)
    if abs(1 - bg/guess) < 0.001
      return guess
    end
    return sqrt_iter(bg)
  end

  return sqrt_iter(x/2)
end

println(csqrt(0.0005))
println(sqrt(0.0005))
