s := [ |2x| for x <- (-2,-1..) if x^2 > 3 ]

s:=[2x for x<-(0..) if x^2>3]
{
    IDENTIFIER: s,
    ASSIGNMENT,
    DELIMITER: '[',
}

s := [ 2x for x in [0..] if x**2 > 3 ]

(x, y) = 5 +- sqrt 4
3 . 4 = 12

~(~A) <=> A
x /= y <=> ~(x = y)

Integer ~ Float

factorial :: a ~ Integral => a -> a

f: Int -> Nat U 0
f(x) := x^2

f :: Num -> Num
f: x -> x + 1

factorial :: a <: Integral
Blackbird <: Bird

plus :: Int Int -> Int;
plus(x, y) := {
    match (y) {
        (0)    -> { x; };
        (S(n)) -> { S(x + n); };
    };
};

xs := [3..100)
a := [x^2, y^2 <- (..), xs : even? x]

x := [1, 2, 3]
y := {4, 6, 8}
println #x == #y -- true

S = { (x, y) : 0 < y < f(x) }
-- implicit (x, y) <- (..) ?

(f o g) x

a := {1, 2, 3}
b := {3, 4, 5}
a v b = {1, 2, 3, 4, 5} -- true
a ^ b = {3} -- true

a := T
b := F
b := T
a v b = true -- true
a ^ b = false

for (2x <- [0..100) : x^2 > 3)

map (\x. 2*x) (filter (\x. x^x > 3) [0..100))

[0..100).filter(\x^x > 3).map(\x*2)

[x*2 <- [0..100) : x^x > 3]
[x * y : x <- [] : y <- [] : x + y > 20]

[1..100].map(println o fizzbuzz)

(a..b) = { x <- (..) : a < x < b }
(a..b] = { x <- (..) : a < x <= b }

<a, b> = <3, 4>

sqrt(3^2 + (-4)^2)
sqrt $ 3^2 + (-4)^2

abs(x) := {
    x,  if x >= 0
    -x, if x < 0
}

trait Shape := {
    .perimeter :: Float;
    .area :: Float;
}

type Circle(radius) := {
    radius :: Float;
}

instance Shape for Circle := #{
    .perimeter :: Float;
    .perimeter := (* 2.0 radius PI);

    .area :: Float;
    .area := (* (PI) (^ radius 2.0));
.#};

data Tree := {Empty|Leaf Int|Node Tree Tree};

type Tree := {
     Empty | Leaf(Int) | Node(Tree, Tree);
} deriving Eq Show;

type Point := {
    x :: Int,
    x :- 0, -- default

    y :: Int :- 0.
} deriving Eq Show.

type Tree := {
    Empty
  | Leaf { item :: Int; }
  | Node { left :: Tree; right :: Tree; };
};

p  :- Point 2 3.
p' :- Point (x :- 4 y :- 5).

t  :- Node Empty Empty
t1 :- Node(Leaf(1), Node(Leaf(2), Leaf(3)));

alias Weekday :- [1..7];


-- subtypes are constraints on types
-- body is a comma delimited set of booleans
type Weekday of Int :- {
    Int > 0
    Int `mem?` [1..7]
};

type Nat of Num :- {
    Num > 0,
    floor Num == Num
};

-- partially abstract type
-- these can hold traits as well as implement them
type Bird :- {
    trait Sing :- {
        sing :: String
    },

    move :: String :- "flap flap.."

    describe :: String
    describe :- f"i say {.sing} while i {.move}"

} deriving Show Eq;

a /\ b
b \/ b

jay :- Bird
println jay.move -- flap flap

type Blackbird of Bird :- {
    Bird.sing = "tra-la-la", -- note equals sign
    -- .move is flap flap
} deriving Ord;

type Ostrich of Bird :- {
    Bird.sing = "hiss hiss!",
    Bird.move = "trot trot..",
    color :- "pink"
};

-- methods go here :)
instance Ostrich :- {
    attack :: Ostrich -> Maybe (String)
    attack o :- match o.sing {
        msg :- "[A-Z]+"  => {
            Just f"How dare you yell {msg} at me!"
        },
        "[a-z]+" => {
            Just "Speak up, I can't hear you!"
        },
        otherwise => Nothing
    }
}

bb :- Blackbird -- nullary type constructor
println bb.describe -- i say tra-la-la while i flap flap

println Ostrich.describe -- i say hiss hiss! while i trot trot..

x :- 3 :: Nat 
y :- Nat 3.5

f :: Nat -> [Nat]
f n :- [1..n)

z :: [Nat]
z :- f x

friday :- 5 :: Weekday
monday :- Weekday 1

type Sort-Args :- {
    cmp :: String :- "lex",
    col :: Int    :- 0,
    rev :: Bool   :- False
};

sorter :: Sort-Args, [[String]] -> [[String]]
sorter (Sort-Args (cmp col rev:False)) table :- {
    let { 
        res :- sort ((\xy. x < cmp < y) filter (== (table `index` col)) table).

        res :- table
            |> { filter (== (index table col)). }
            |> { sort (\xy: x < cmp < y) }.

        res :- table:filter (== (index table col)):sort (\xy: x < cmp < y).
    } in {
        if { rev } then {
            reverse (res).
        } else {
            res.
        }.
    }.
}.

sorter (Sort-Args (cmp col rev:False)) table :- {
    if { rev. } then {
        reverse res.
    } else {
        res.
    },
    where {
        res :- table
            |> { filter (== (index table col)). } 
            |> { sort (\xy: x < cmp < y). }.
    }.
}.

trait Eq :- {
    equals? _ _ -> Bool,
    equals? a b :- { ~(neq? a b). },

    neq? _ _ -> Bool,
    neq? a b :- { ~(equals? a b). }.
}.

type Point :- {
    x :: Int,
    x :- 0,

    y :: Int,
    y :- 0.
}.

type Tree :- {
    Empty
    | Leaf Int
    | Node Tree Tree.
}.

type Tree :- {
    Empty
    | Leaf (item :: Int)
    | Node (left :: Tree, right :: Tree).
}

type Point :- {
    Point (x :: Int, y :: Int).
}

type Expr :- { Expr. }.

x :- [1, 2, 3].
x :- Cons 1 (Cons 2 (Cons 3 Nil)).
y :- 3.
z :- 3.45.
a :- 'a'.
b :- (\x. x + -1)

6.3.14.2.

[1,2.3,6,5.2].[4,4.4,6.6,8]
6. 3.14. 2.'

'a'. 'b'. 'c'

(\x. x + 1) 6.

{
    6,
    3.14,
    'a',
    + 3 2,
    + 8 (+ 3 4),
    (\x. x),
    (\x. x + 1) 6,
    (\x. \y. x + -y) 6 3.
}





