import D01.Spec qualified
import D02.Spec qualified
import D03.Spec qualified
import D04.Spec qualified
import D05.Spec qualified
import Test.Hspec

main :: IO ()
main = hspec $ do
  D01.Spec.spec
  D02.Spec.spec
  D03.Spec.spec
  D04.Spec.spec
  D05.Spec.spec
