module D01.Spec (day) where

import Days (Day, Day' (Day))
import Days qualified as Part
import Days.D01 qualified as D01

day :: Day
day =
  Day
    1
    D01.part1
    D01.part2
    [ (Part.One, "example.txt", 3),
      (Part.One, "input.txt", 1165),
      (Part.Two, "example.txt", 6),
      (Part.Two, "input.txt", 6496)
    ]
