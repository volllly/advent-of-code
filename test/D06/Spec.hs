module D06.Spec (day) where

import Days (Day, Day' (Day))
import Days qualified as Part
import Days.D06 qualified as D06

day :: Day
day =
  Day
    6
    D06.part1
    D06.part2
    [ (Part.One, "example.txt", 4277556),
      (Part.One, "input.txt", 5733696195703),
      (Part.Two, "example.txt", 3263827),
      (Part.Two, "input.txt", 10951882745757)
    ]
