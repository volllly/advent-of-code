module D07.Spec (day) where

import Days (Day (..))
import Days qualified as Part
import Days.D07 qualified as D07

day :: Day
day =
  Day
    7
    D07.part1
    D07.part2
    [ (Part.One, "example.txt", 21),
      (Part.One, "input.txt", 1687),
      (Part.Two, "test.txt", 3),
      (Part.Two, "example.txt", 40),
      (Part.Two, "input.txt", 390684413472684)
    ]
