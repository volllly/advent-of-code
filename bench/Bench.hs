import D01.Spec qualified
import D02.Spec qualified
import D03.Spec qualified
import D04.Spec qualified
import D05.Spec qualified
import D06.Spec qualified
import D07.Spec qualified
import D08.Spec qualified
import Days (Day' (..), solver)
import System.IO.Unsafe (unsafePerformIO)
import Test.Tasty.Bench
import Text.Printf (printf)

benchmark :: Day' a -> Benchmark
benchmark day =
  bgroup (show day) $
    foldMap
      ( \(part, name, args, _) ->
          let fileName = "test/D" ++ printf "%02d" (number day) ++ "/" ++ name
              fileContent = unsafePerformIO $ readFile fileName
           in [ bgroup
                  (show part)
                  [ bench name $ nf (solver part day args) fileContent
                  ]
              ]
      )
      (cases day)

main :: IO ()
main =
  defaultMain
    [ bgroup
        "Advent of Code"
        [ benchmark D01.Spec.day,
          benchmark D02.Spec.day,
          benchmark D03.Spec.day,
          benchmark D04.Spec.day,
          benchmark D05.Spec.day,
          benchmark D06.Spec.day,
          benchmark D07.Spec.day,
          benchmark D08.Spec.day
        ]
    ]
