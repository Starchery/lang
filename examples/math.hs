arithmetic :: Int Int -> ()
arithmetic x y =
  [+ - * / ^ div mod]
    .map \op.
      println f"{x} {op} {y} => {op x y}"

x + y
x - y
x * y
x / y
x // y
x ^ y

trait Shape a =
  perimeter :: a -> Float
  area :: a -> Float

data Rectangle =
  width :: Integer,
  height :: Integer

instance Shape for Rectangle where
  perimeter (Rectangle w h) =
    2 * w * h
  
  area (Rectangle w h) = w * h

type Rectangle width height =
  instance Shape
    perimeter = 2 * width * height
    area = width * height

instance Shape for Rectangle =
  perimeter :: Float
  perimeter = 2 * width * height

  area :: Float
  area = width * height

type Bird =
  trait move :: ()
  move = println "flying"

  trait sing :: String

type Blackbird deriving Bird =
  instance sing :: String
  sing = "tra-la-la"

type Ostrich deriving Bird =
  instance move = println "walking"
  instance sing = "hiss hiss!"

trait Eq T =
  (=) :: Bool
  (~=) :: Bool
  
  T = T = not (T ~= T)
  T ~= T = not (T = T)

type Rectangle :=
  width :: Int
  height :: Int
  
  instance
    Eq
      (=) 
        (Rectangle w1 h1) 
        (Rectangle w2 h2)
        :=
          w1 = h1 ^ w2 = h2
      (~=) r1 r2 <- ~ (r1 = r2)




