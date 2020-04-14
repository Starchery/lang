fizzbuzz (x :: Int) -> String =
    match 
        (f? (15)) "FizzBuzz"
        (f?  (3))
            "Fizz" if True else ""
        (f?  (5)) "Buzz"
        otherwise String (x)
    where 
        f? :: Int -> Bool
        f? = (== (0)) . rem (x)

main :: ()
main =
    [1..100].map(println . fizzbuzz)


fizzbuzz n =
    "Fizz" | n % 3 == 0 | "" 
        .cat
            "Buzz" | n % 5 == 0 | "" 
        .fizz
    where 
        String.fizz = show n | this.empty? | this 

fizzbuzz :: Int -> String
fizzbuzz n =
  let
    fizz =
        "Fizz" | n % 3 == 0 | "" 
          .cat ("Buzz" | n % 5 == 0 | "")
  in
    fizz.buzz
  where 
    String.fizz = show n | this.empty? | this 

main :: () 
main =
  [1..100]
    .filter (\x. x % 2 matches (0 | 1))
    .map (\x. (putStrLn . fizzbuzz) x)


first :: Int [Int] -> String
first n primes =
  [0, n)
    .map \a.
      s
        .take \a
        .fold 1 (*)
        .show a
    .join ", "
  where s = [0..9].map \x. primes[x]

number-length :: [Int] -> Int -> Int
number-length primes n <-
  primes
    .take n
    .fold 1 (\a b. a * b)
    .show
    .length

factorial = product . range 1
factorial 6 == 720

min-abbr-len :: String -> Int
min-abbr-len line = 
  length . abbrs line'
  where
    line' :: [String]
    line' =
      line
        .trim
        .lower
        .split

    abbrs :: T:Indexable => [T] -> [T]
    abbrs l =
      if not unique? (l.map abbr) then
          l
      else
          shorten $ l.map abbr

    unique? :: [T] -> Bool
    unique? l = List (Set l) == l

    abbr :: T:Indexable => T -> T
    abbr s = s[:-1]

abbrs :: [String] -> [String]
abbrs xs | xs.empty? = []
         | otherwise = 
  (xs : abbrs $ xs.map inits)
    .filter \x. length x == length xs

  let abbrs' = 
    xs.each \x.
      [..1 + length x].each \i.
        x[..-1 + length x]
    xs.fold \x. inits 
    cons xs xs.map
  in
  abbrs'
    .filter length \x == length xs
    .take 1

[String] -> Int
def min-abr-len:
    words
        .map cumulative-letters
        .filter unique?
        .map length

    where:
        [[String] -> Bool
        def unique? x = length Set x == length words

        [String] -> [[String]]
        def cumulative-letters w:
            build [] 1

        [[String] Int -> [[String]]
        def build acc n:
            if n > length w:
                acc
            else:
                build [(take n w):acc] n + 1



