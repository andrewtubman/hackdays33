def fibonacci(number)
  return number if number < 2

  i = 0
  f = 0
  l = 1

  while i < number
    i += 1;
    f = f + l;
    l = f - l;
  end
  
  f
end

puts "Find the nth fibonacci number"
puts "Enter a number:"

number = 55;

fib = fibonacci(number);

puts fib

