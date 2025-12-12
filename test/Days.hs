module Days (Day (..), Part (..), test) where

import Data.Foldable
import Test.Hspec
import Text.Printf

data Day = Day
  { number :: Int,
    part1 :: String -> Int,
    part2 :: String -> Int
  }

solver :: Part -> Day -> (String -> Int)
solver One = part1
solver Two = part2

instance Show Day where
  show day = "Day " ++ show (number day)

data Part = One | Two

instance Show Part where
  show One = "Part 1"
  show Two = "Part 2"

test :: Day -> [(Part, String, Int)] -> Spec
test day tests =
  describe (show day) $
    for_ tests $ \(part, name, expected) ->
      let fileName = "test/D" ++ printf "%02d" (number day) ++ "/" ++ name
       in describe (show part) $
            it name $ do
              fileContent <- readFile fileName
              solver part day fileContent `shouldBe` expected
