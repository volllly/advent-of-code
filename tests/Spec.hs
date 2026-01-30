import D01.Spec qualified
import D02.Spec qualified
import D03.Spec qualified
import D04.Spec qualified
import D05.Spec qualified
import D06.Spec qualified
import D07.Spec qualified
import D08.Spec qualified
import Days (Day' (..), solver)
import Test.Tasty
import Test.Tasty.HUnit (testCase, (@?=))
import Text.Printf (printf)

test :: Day' a -> TestTree
test day =
  testGroup (show day) $
    foldMap
      ( \(part, name, args, expected) ->
          let fileName = "test/D" ++ printf "%02d" (number day) ++ "/" ++ name
           in [ testGroup
                  (show part)
                  [ testCase name $ do
                      fileContent <- readFile fileName
                      solver part day args fileContent @?= expected
                  ]
              ]
      )
      (cases day)

main :: IO ()
main =
  defaultMain $
    testGroup
      "Advent of Code Tests"
      [ test D01.Spec.day,
        test D02.Spec.day,
        test D03.Spec.day,
        test D04.Spec.day,
        test D05.Spec.day,
        test D06.Spec.day,
        test D07.Spec.day,
        test D08.Spec.day
      ]
