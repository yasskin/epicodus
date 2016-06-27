Ruby Loops

Code Samples

Example simple until loop:

answer = ""
until answer == "no" do
  print "Do you want this loop to continue? (y/n) "
  answer = gets.chomp
end

Example until loop used with a number based conditional exit:

def print_hello(number_of_times)
  i = 0
  while i < number_of_times
    puts "hello"
    i += 1
  end
end

answer = 0
until answer >= 5
  print "How many times do you want to print 'hello'? Enter a number greater than 5 to exit) "
  answer = gets.chomp.to_i
  print_hello(answer)
end

Concepts

Iterator variable

Code Samples

Here is a simple while loop:

answer = ""
while answer != "n"
  print "Do you want me to repeat this pointless loop again? (y/n) "
  answer = gets.chomp.downcase
end
Example while loop with exit conditional as a number:

def print_hello(number_of_times)
  i = 0
  while i < number_of_times
    puts "hello"
    i += 1
  end
end

answer = 0
while answer < 5
  print "How many times do you want to print 'hello'? Enter a number greater than 5 to exit) "
  answer = gets.chomp.to_i
  print_hello(answer)
end

Code Samples

Random number guessing program:

loop_conditional.rb

random_number = Random.new.rand(5)
loop do
  print "Guess the number between 0 and 5 (e to exit): "
  answer = gets.chomp
  if answer == "e"
    puts "The number was #{random_number}."
    break
  else
    if answer.to_i == random_number
      puts "You guessed correctly!"
      break
    else
      puts "Try again."
    end
  end
end

Program to exit a loop when a number greater than 10 is entered:

loop_conditional_number.rb

loop do
  print "Enter a number greater than 10 to exit: "
  answer = gets.chomp.to_i
  break if answer > 10
end

Program to loop through asking for someone's name and make sure it is formatted correctly:

def get_name
  name = ""
  loop do
    print "Enter your name (minimum 2 characters, no numbers): "
    name = gets.chomp
    if name.length >= 2 && !name.index(/\d/)
      break
    else
      puts "Name must be longer than 2 characters and not contain numbers."
    end
  end
  return name
end

name = get_name()
puts "Hi #{name}."
