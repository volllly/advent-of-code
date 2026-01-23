module Days.D07 (part1, part2) where

import Data.Function ((&))
import Data.IntMap.Strict (IntMap)
import Data.IntMap.Strict qualified
import Data.IntSet (IntSet)
import Data.IntSet qualified
import Data.List (elemIndex, elemIndices)
import Data.List.NonEmpty (nonEmpty)
import Data.List.NonEmpty qualified as NonEmpty
import Data.Maybe (fromJust)

parseInput :: String -> (Int, [[Int]])
parseInput input =
  let inputLines = fromJust $ nonEmpty [x | (x, i) <- zip (lines input) [(0 :: Int) ..], even i]
      start = NonEmpty.head inputLines & elemIndex 'S' & fromJust
      splitters = NonEmpty.tail inputLines & map (elemIndices '^')
   in (start, splitters)

part1 :: String -> Int
part1 input =
  let (start, splitters) = parseInput input
      split (splits, beams) splitter =
        let newBeams =
              beams
                & Data.IntSet.toList
                & map
                  ( \b ->
                      if b `elem` splitter
                        then [b - 1, b + 1]
                        else [b]
                  )
            newSplits = newBeams & filter (\beam -> length beam == 2) & length
         in (splits + newSplits, Data.IntSet.fromList (concat newBeams))
   in foldl split (0, Data.IntSet.singleton start) splitters & fst

splitBeams :: Int -> [IntSet] -> IntMap Int -> Int
splitBeams start [] beams =
  fromJust $ Data.IntMap.Strict.lookup start beams
splitBeams start rows beams =
  let next = last rows
      rest = init rows
      newBeams =
        beams
          & Data.IntMap.Strict.toList
          & map
            ( \(b, v) ->
                case ((b - 1) `Data.IntSet.member` next, (b + 1) `Data.IntSet.member` next) of
                  (True, True) -> [(b - 1, v), (b + 1, v)]
                  (True, False) -> [(b - 1, v)]
                  (False, True) -> [(b + 1, v)]
                  (False, False) -> []
            )
          & concat
          & Data.IntMap.Strict.fromListWith (+)
      remainingBeams = beams & Data.IntMap.Strict.filterWithKey (\b _ -> b `Data.IntSet.notMember` next)
      updatedBeams = Data.IntMap.Strict.unionWith (+) remainingBeams newBeams
   in splitBeams start rest updatedBeams

part2 :: String -> Int
part2 input =
  let (start, splitters) = parseInput input
      toSets = map Data.IntSet.fromList
      width = (last $ last splitters) + 2
      beams = zip [0 ..] (take width $ repeat 1) & Data.IntMap.Strict.fromList
   in splitBeams start (toSets splitters) beams
