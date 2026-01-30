module D08.Spec (day) where

import Days (Day' (..))
import Days qualified as Part
import Days.D08 qualified as D08

day :: Day' Int
day =
  Day'
    8
    D08.part1
    D08.part2
    [ (Part.One, "example.txt", 10, 40),
      (Part.One, "input.txt", 1000, 63920),
      (Part.Two, "example.txt", 0, 25272),
      (Part.Two, "input.txt", 0, 1026594680)
    ]
