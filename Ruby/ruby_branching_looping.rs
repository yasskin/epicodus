* Ruby Branching and Looping

* Practice: Ping-Pong Test



* Practice: More Complex Branching

Now, play around with elsif, &(), |(), and !() to get the hang of how to use them.
Create a variable called grade and set it equal to A, B, C, D or F. Use the above tools to print out one message if the grade is an A, one message if the grade is a B or a C, and one message if the grade is D or F.
Create a variable called temperature and set it equal to a Fixnum to represent a number of degrees Fahrenheit. Check to see if temperature is within 60-80 degrees, and if it is then print out "It's balmy outside!", and a different message if it isn't. You will have to use a combination of a few of these: <() (less-than), <=() (less-than-or-equal-to), >() (greater-than), >=() (greater-than-or-equal-to) and &().
Create a variable to store your first name. Then use if and else to print one message if your name begins with a letter between A and M, and a different message if it begins with a letter between N and Z. Hint: Remember the String#split method and the Array#at method from the section on "Arrays and ranges". Also, you can check if a number or letter is within a range by using the Range#include? method. For example: (0..4).include?(1) will return true because 1 is between 0 and 4.


For more complex branching conditions, you can use the elsif keyword after the if but before the end keywords. The elsif keyword allows many different conditions to be evaluated in the if/else statement. You can use as many elsif statements as you need.
The & method ('and' method) and the | method ('or method') can be used to evaluate multiple values in an if/else statement. In the & method, both the receiver and the argument need to be true for the statement to return true. In the | method, either the receiver or the argument can be true for the condition to return true.
The ! method ('not method') returns the opposite boolean of what it was called on. Here is an example: if young.&(male).!() and this is read "If you are not young and male".

> age = 25
> gender = :male

> young = age.<(26)
> male = gender.eql?(:male)

> if young.&(male)
>   "Your insurance is going to be so expensive!!!"
> elsif young.|(male)
>   "Your insurance is going to be pretty expensive."
> else
>   "Your insurance is going to be expensive, but it could be worse."
> end


* Practice: Methods Using Branching and Booleans

Make an Fixnum#absolutely_larger method that will add 1 to a number if it is positive or 0, and subtract 1 if it is negative.

class Fixnum
  define_method(:absolutely_larger?) do
    if self.>0
      self + 1
    else
      self.-1
    end
  end
end

Make an Fixnum#can_drink_alcohol? method that returns a boolean based on if the Fixnum is greater than or equal to 21.

class Fixnum
  define_method(:can_drink_alcohol?) do
    self.>=(21)
  end
end

Make an Fixnum#has_two_digits? method which returns true if the Fixnum is between 10 and 99, or -10 and -99.

class Fixnum
  define_method(:has_two_digits?) do
    (self.>=(10) && self.<=(99)) || (self.<=(-10) && self.<=(-99))
  end
end

> class String
*   define_method(:starts_with_vowel?) do
*     vowels = ["a", "e", "i", "o", "u"]
*     letters = self.split("")
*     first_letter = letters.first()
*     vowels.include?(first_letter)
*   end
* end

> "apple".starts_with_vowel?()
=> true

abs means absolute value
> class Fixnum
*   define_method(:abs) do
*     if self.>=(0)
*       self
*     else
*       self.*(-1)
*     end
*   end
* end

> -5.abs()
=> 5

* Practice: Boolean Objects

Integer:
even?()
odd?()

Integer and Float:
<(number) # takes an argument of another number and tells if the first is less than the second
>(number)
<=(number)
>=(number)
==(number) # takes an argument of another number and tells if the two are equal

Array:
empty?()
include?(object) # takes an object as an argument and tells if it is in the array

Range:
cover?(object) # takes an object as an argument and tells if it falls within the range

.odd?()
.<()
.cover?()
.class()
.eql?() // for strings

* Practice: Methods Using Loops

Make an upcase! method on Array that changes the receiver.

class Array
  define_method(:upcase) do
    new_array = []
    self.each() do |string|
      upcased_string = string.upcase!()
      new_array.push(upcased_string)
    end
    new_array
  end
end

Make a product method on Range that multiples the numbers in the range.

class Range
  define_method(:multiplier) do
  total = 1
  self.each() do |number|
    total = total.*(number)
    end
    total
  end
end

Make a String method called triple_each_letter(). For example, if I start with the String "Hello" I want to be able to call "Hello".triple_each_letter() and have it return "HHHeeellllllooo".

class String
  define_method(:trippler) do
  split_letters = self.split("")
  new_letters = []
  split_letters.each() do |letter|
    push.
  end


Turn the method you just wrote, triple_each_letter(), into a new method called duplicate_each_letter() which takes a Fixnum argument for the number of times to duplicate each letter in the String that we call the method on. For example, if I call "Hi".duplicate_each_letter(5) it should return "HHHHHiiiii". Or, "Epicodus".duplicate_each_letter(2) would give me "EEppiiccoodduuss".

methods that use loops. Here's an example using a method that takes an Array of Strings and turns them all uppercase:

> class Array
*   define_method(:upcase) do
*     new_array = []
*     self.each() do |string|
*       upcased_string = string.upcase()
*       new_array.push(upcased_string)
*     end
*     new_array
*   end
* end

> ["a", "b", "c"].upcase()
=> ["A", "B", "C"]

Here's another example, using a method that adds a range of numbers:

> class Range
*   define_method(:sum) do
*     total = 0
*     self.each() do |number|
*       total = total.+(number)
*     end
*     total
*   end
* end

> (1..5).sum()
=> 15

* Practice: Each Loops

Loop through a list of your friends and say they are your friend (for example, "Moriah is my friend.").

friends = ["Terry", "Ben", "David"]
friends.each() do |friend|
  friend.concat(" is my friend.")
end
friends

Take a range of numbers and multiply them together.

numbers = (1..3)
product = 1
numbers.each() do |number|
  product = product.*(number)
end
product

Create a variable called my_fave_drink and set it equal to a String. Also create a variable to hold an empty Array and call it my_new_drink. Convert my_fave_drink to an Array of single letters. Now loop through each letter stored in that Array and push it into the my_new_drink Array three times. Then convert my_new_drink into a String, and set my_fave_drink equal to this new String. So if your favorite drink is "Pepsi", at the end of the loop it should be "PPPeeepppsssiii".

my_fave_drink = "Coke"
my_new_drink = []
my_fave_drink.split(//)
  my_fave_drink.each() do |letter|
    my_new_drink = my_fave_drink.push(letter)
  end
my_fave_drink = my_new_drink.join()


Below is an example each loop:
friends = ["Liz", "Sal", "Jo"]
friends.each() do |friend|
  friend.concat(" loves programming!")
end
friends

Practice: Loops

Try removing the last 4 elements of an Array.

elements = [1, 2, 3, 4, 5, 6, 7, 8]
4.times() do
  new_elements = elements.pop()
end
elements

Add a number to itself several times.

number = 0
6.times() do |time|
  number = number.+(time)
end
number

Use a parameter with a times loop to create an Array that looks like this: [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4]

numbers = []
5.times() do |time|
  numbers.push(time)
  numbers.push(time)
  numbers.push(time)
end
numbers
