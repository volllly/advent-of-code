module D02.Spec (day) where

import Days (Day, Day' (Day))
import Days qualified as Part
import Days.D02 qualified as D02

day :: Day
day =
  Day
    2
    D02.part1
    D02.part2
    [ (Part.One, "example.txt", 1227775554),
      (Part.One, "input.txt", 38310256125),
      (Part.Two, "example.txt", 4174379265),
      (Part.Two, "input.txt", 58961152806)
    ]
