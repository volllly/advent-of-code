module D04.Spec (spec) where

import Days (Day (..), test)
import Days qualified as Part
import Days.D04 qualified as D04
import Test.Hspec (Spec)

day :: Day
day = Day 4 D04.part1 D04.part2

spec :: Spec
spec =
  test
    day
    [ (Part.One, "example.txt", 13),
      (Part.One, "input.txt", 1551),
      (Part.Two, "example.txt", 43),
      (Part.Two, "input.txt", 9784)
    ]
