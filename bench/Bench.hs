import D01.Spec qualified
import D02.Spec qualified
import D03.Spec qualified
import D04.Spec qualified
import D05.Spec qualified
import Days (Day (..), solver)
import System.IO.Unsafe (unsafePerformIO)
import Test.Tasty.Bench
import Text.Printf (printf)

benchmark :: Day -> Benchmark
benchmark day =
  bgroup (show day) $
    foldMap
      ( \(part, name, _) ->
          let fileName = "test/D" ++ printf "%02d" (number day) ++ "/" ++ name
              fileContent = unsafePerformIO $ readFile fileName
           in [ bgroup
                  (show part)
                  [ bench name $ nf (solver part day) fileContent
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
          benchmark D05.Spec.day
        ]
    ]
