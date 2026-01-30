module Days.D08 (part1, part2) where

import Data.Containers.ListUtils (nubOrdOn)
import Data.Function ((&))
import Data.List (findIndex, sort, sortOn)
import Data.List.Split (splitOn)
import Data.Maybe (fromJust, fromMaybe)
import Data.Set (Set (..), member)
import Data.Set qualified
import Linear (Metric (distance))
import Linear.V3 (V3 (..))

parseInput :: String -> [V3 Int]
parseInput input =
  map (\t -> let [x, y, z] = map read (splitOn "," t) in V3 x y z) $ lines input

part1 :: Int -> String -> Int
part1 amount input =
  let junctions = parseInput input
      distances from = map (\j -> (distance (fmap fromIntegral from) (fmap fromIntegral j), (from, j))) junctions & filter (\(d, _) -> d /= 0)
      nearest = junctions & map distances & concat & nubOrdOn (\(_, (a, b)) -> if a > b then (a, b) else (b, a)) & sortOn (\(d, _) -> d) & map snd
      groups = groupJunctions amount nearest $ map Data.Set.singleton junctions
   in product $ take 3 $ reverse $ sort $ map length groups

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
      distances from = map (\j -> (distance (fmap fromIntegral from) (fmap fromIntegral j), (from, j))) junctions & filter (\(d, _) -> d /= 0)
      nearest = junctions & map distances & concat & nubOrdOn (\(_, (a, b)) -> if a > b then (a, b) else (b, a)) & sortOn (\(d, _) -> d) & map snd
      ((V3 x1 _ _, V3 x2 _ _), _) = groupAllJunctions nearest Nothing $ map Data.Set.singleton junctions
   in x1 * x2