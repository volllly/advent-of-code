module D02.Spec (spec) where

import Days (Day (..), test)
import qualified Days as Part
import qualified Days.D02 as D02
import Test.Hspec (Spec)

day :: Day
day = Day 2 D02.part1 D02.part2

spec :: Spec
spec =
  test
    day
    [ (Part.One, "example.txt", 1227775554),
      (Part.One, "input.txt", 38310256125),
      (Part.Two, "example.txt", 4174379265),
      (Part.Two, "input.txt", 58961152806)
    ]
