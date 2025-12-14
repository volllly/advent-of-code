module D03.Spec (day) where

import Days (Day (..))
import Days qualified as Part
import Days.D03 qualified as D03

day :: Day
day =
  Day
    3
    D03.part1
    D03.part2
    [ (Part.One, "example.txt", 357),
      (Part.One, "input.txt", 17085),
      (Part.Two, "example.txt", 3121910778619),
      (Part.Two, "input.txt", 169408143086082)
    ]
