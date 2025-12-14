module Days (Day (..), Part (..), solver) where

data Day = Day
  { number :: Int,
    part1 :: String -> Int,
    part2 :: String -> Int,
    cases :: [(Part, String, Int)]
  }

solver :: Part -> Day -> (String -> Int)
solver One = part1
solver Two = part2

instance Show Day where
  show day = "Day " ++ show (number day)

data Part = One | Two

instance Show Part where
  show One = "Part 1"
  show Two = "Part 2"
