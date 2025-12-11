module D01.Spec (spec) where

import Days (Day (..), test)
import qualified Days as Part
import qualified Days.D01 as D01
import Test.Hspec (Spec)

day :: Day
day = Day 1 D01.part1 D01.part2

spec :: Spec
spec =
  test
    day
    [ (Part.One, "example.txt", 3),
      (Part.One, "input.txt", 1165),
      (Part.Two, "example.txt", 6),
      (Part.Two, "input.txt", 6496)
    ]
