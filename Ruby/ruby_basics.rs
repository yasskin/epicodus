Ruby Basics

Ruby Basics: Practice Defining Methods with Arguments

class Fixnum
  define_method(:subtract) do |number_to_subtract|
    self.-(number_to_subtract)
  end
end

class String
  define_method(:combine) do |string_to_concat|
    self.+(string_to_concat)
  end
end



Ruby Basics: Practice Defining Methods

class Fixnum
  define_method(:previous) do
    self.-(1)
  end
end

class Fixnum
  define_method(:gimme_ten) do
    10
  end
end

class Float
  define_method(:half) do
    self./(2)
  end
end

class Array
  define_method(:next_in_line) do
    self.shift().push()
  end
end

class Array
  define_method(:trim) do
    self.shift().pop()
  end
end

class Fixnum
  define_method(:add) do |number_to_add|
    self.+(number_to_add)
  end
end
