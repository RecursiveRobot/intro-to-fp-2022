#region 0. Boilerplate
from functools import partial, reduce
from itertools import groupby
from pyfluent_iterables import fluent
import math
from dataclasses import dataclass
#endregion

# Partially Applied/Curried Functions
add2numbers = lambda a, b: a + b
add2numbersPartial = lambda a: lambda b: a + b
add2numbersCurried = partial(add2numbers)
plus5 = add2numbersPartial(5)
twelve = plus5(7)
fortyTwo = add2numbersPartial(15)(27)

# Composition
def plus12(x: int): return x + 12
def mult5(x: int): return x * 5
compose = lambda f, g: lambda x: f(g(x))
plus12mult5 = compose(mult5, plus12)

plus12mult5(1)

# Higher-Order Functions
characters = \
[
  "Rusty Venture",
  "Brock Sampson",
  "Dean Venture",
  "Hank Venture",
  "Helper Robot",
  "Dr. Orpheus",
  "The Monarch",
  "Dr. Girlfriend"
]

isDirectFamily = lambda name: name.contains("Venture")
directFamily = filter(isDirectFamily, characters)
doctorCount = len(filter(lambda n: n.contains("Dr."), characters))

charactersByLastName = fluent(characters) \
  .map(lambda n: n.split(" ")[1]) \
  .group_by(lambda n: n) \
  .map_values(len).to_dict()

# Pattern matching
@dataclass
class Shape(): ()
@dataclass
class Circle(Shape):
  radius: float
@dataclass
class Rectangle(Shape):
  width: float
  height: float
@dataclass
class Triangle(Shape):
  side1: float
  side2: float
  side3: float

def getArea(shape: Shape):
  match shape:
    case Circle(r):
      return math.pi * r**2
    case Rectangle(w, h):
      return w * h
    case Triangle(a,b,c):
      s = (a + b + c) / 2
      return math.sqrt(s*(s-a)*(s-b)*(s-c))

getArea(Circle(2.5))
getArea(Rectangle(10, 20))
getArea(Triangle(2,3,4))
