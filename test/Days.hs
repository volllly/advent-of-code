{-# LANGUAGE PatternSynonyms #-}
{-# LANGUAGE ViewPatterns #-}

module Days (Day, Day' (.., Day), Part (..), solver) where

type Solver a = a -> String -> Int

type Cases a = (Part, String, a, Int)

data Day' a = Day'
  { number :: Int,
    part1 :: Solver a,
    part2 :: Solver a,
    cases :: [Cases a]
  }

type Day = Day' ()

pattern Day :: Int -> (String -> Int) -> (String -> Int) -> [(Part, String, Int)] -> Day
pattern Day n p1 p2 cs <- Day' n ((\f -> f ()) -> p1) ((\f -> f ()) -> p2) (map dropUnit -> cs)
  where
    Day n p1 p2 cs = Day' n (const p1) (const p2) (map addUnit cs)

dropUnit :: (Part, String, (), Int) -> (Part, String, Int)
dropUnit (p, s, (), i) = (p, s, i)

addUnit :: (Part, String, Int) -> (Part, String, (), Int)
addUnit (p, s, i) = (p, s, (), i)

solver :: Part -> Day' a -> Solver a
solver One = part1
solver Two = part2

instance Show (Day' a) where
  show day = "Day " ++ show (number day)

data Part = One | Two

instance Show Part where
  show One = "Part 1"
  show Two = "Part 2"
