module Days.D08 (part1, part2) where

import Data.Containers.ListUtils (nubOrdOn)
import Data.Function ((&))
import Data.Functor ((<&>))
import Data.List (findIndex, sort, sortOn)
import Data.List.Split (splitOn)
import Data.Maybe (fromJust, fromMaybe)
import Data.Set (Set, member)
import Data.Set qualified
import Linear (Metric (distance))
import Linear.V3 (V3 (..))

parseInput :: String -> [V3 Int]
parseInput input =
  ( \t -> case read <$> splitOn "," t of
      [x, y, z] -> V3 x y z
      _ -> undefined
  )
    <$> lines input

ordered :: [V3 Int] -> [(V3 Int, V3 Int)]
ordered junctions =
  let calculateDistances :: V3 Int -> [(Double, (V3 Int, V3 Int))]
      calculateDistances from = [(d, (from, j)) | j <- junctions, let d = distance (fromIntegral <$> from) (fromIntegral <$> j), d /= 0]
      withDistances = junctions <&> calculateDistances & concat
   in withDistances & nubOrdOn (\(_, (a, b)) -> (min a b, max a b)) & sortOn fst <&> snd

part1 :: Int -> String -> Int
part1 amount input =
  let junctions = parseInput input
      nearest = ordered junctions
      groups = groupJunctions amount nearest $ map Data.Set.singleton junctions
   in product $ take 3 $ reverse $ sort $ length <$> groups

filterIndexed :: (a -> Int -> Bool) -> [a] -> [a]
filterIndexed p xs = [x | (x, i) <- zip xs [0 ..], p x i]

addNearest :: [Set (V3 Int)] -> (V3 Int, V3 Int) -> (Bool, [Set (V3 Int)])
addNearest groups (a, b) =
  let ai = fromJust $ findIndex (a `member`) groups
      bi = fromJust $ findIndex (b `member`) groups
      new = (groups !! ai) `Data.Set.union` (groups !! bi)
   in if ai == bi then (False, groups) else (True, filterIndexed (\_ i -> (i /= ai && i /= bi)) groups ++ [new])

groupJunctions :: Int -> [(V3 Int, V3 Int)] -> [Set (V3 Int)] -> [Set (V3 Int)]
groupJunctions 0 _ groups = groups
groupJunctions _ [] groups = groups
groupJunctions count (nearest : rest) groups =
  let (_, newGroups) = addNearest groups nearest
   in groupJunctions (count - 1) rest newGroups

groupAllJunctions :: [(V3 Int, V3 Int)] -> Maybe (V3 Int, V3 Int) -> [Set (V3 Int)] -> ((V3 Int, V3 Int), [Set (V3 Int)])
groupAllJunctions [] latest groups = (fromJust latest, groups)
groupAllJunctions (nearest : rest) latest groups =
  let (added, newGroups) = addNearest groups nearest
   in groupAllJunctions rest (Just $ if added then nearest else fromMaybe nearest latest) newGroups

part2 :: Int -> String -> Int
part2 _ input =
  let junctions = parseInput input
      nearest = ordered junctions
      ((V3 x1 _ _, V3 x2 _ _), _) = groupAllJunctions nearest Nothing $ Data.Set.singleton <$> junctions
   in x1 * x2