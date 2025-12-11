import qualified D01.Spec
import qualified D02.Spec
import Test.Hspec

main :: IO ()
main = hspec $ do
  D01.Spec.spec
  D02.Spec.spec
