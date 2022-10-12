-- #region 0. Boilerplate
import Data.List
import Data.Text (pack)



-- #endregion

-- Partially Applied/Curried Functions
add2numbers (a, b) = a + b
add2numbersPartial a b = a + b
add2numbersCurried a b = add2numbers (a, b)
plus5 = add2numbersCurried 5
twelve = plus5 7
fortyTwo = add2numbersPartial 15 27

-- Composition
plus12 = (+) 12
mult5 = (*) 5

plus12mult5 = mult5 . plus12

_ = plus12mult5 1

-- Higher-Order Functions
characters =
  [ 
    "Rusty Venture",
    "Brock Samson",
    "Dean Venture",
    "Hank Venture",
    "Helper Robot",
    "Dr. Orpheus",
    "The Monarch",
    "Dr. Girlfriend"
  ]

isDirectFamily = isInfixOf "Venture"
directFamily = filter isDirectFamily characters
doctorCount = length $ filter (isInfixOf "Dr.") characters

charactersByLastName = fmap formatResult $ group $ sort $ getLastName <$> characters
  where
    getLastName = pack . (!! 1) . words
    formatResult ns = (head ns, length ns)

-- Pattern Matching
data Shape
  = Circle    { radius :: Double }
  | Rectangle { width :: Double, height :: Double }
  | Triangle  { side1 :: Double, side2 :: Double, side3 :: Double }

getArea :: Shape -> Double
getArea (Circle r)       = pi * r^2
getArea (Rectangle w h)  = w * h
getArea (Triangle a b c) = sqrt (s*(s-a)*(s-b)*(s-c)) -- Heron's Formula
  where s = (a + b + c) / 2

_ = getArea(Circle {radius=2.5})
_ = getArea(Rectangle {width=10, height=20})
_ = getArea(Triangle {side1=2, side2=3, side3=4})














