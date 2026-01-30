module D05.Spec (day) where

import Days (Day, Day' (Day))
import Days qualified as Part
import Days.D05 qualified as D05

day :: Day
day =
  Day
    5
    D05.part1
    D05.part2
    [ (Part.One, "example.txt", 3),
      (Part.One, "input.txt", 707),
      (Part.Two, "example.txt", 14),
      (Part.Two, "input.txt", 361615643045059)
    ]
