module Days.D06 (part1, part2) where

import Data.Function ((&))
import Data.List (groupBy, transpose)
import Data.Maybe (catMaybes, fromJust, isNothing)
import Text.Read (readMaybe)

data Operator = Add | Multiply

instance Read Operator where
  readsPrec _ s = case s of
    "+" -> [(Add, "")]
    "*" -> [(Multiply, "")]
    _ -> []

instance Show (Operator) where
  show Add = "+"
  show Multiply = "*"

parseInput1 :: String -> [([Int], Operator)]
parseInput1 input =
  let problems = lines input & map words & transpose
      parse problem =
        let numbers = init problem & map read
            operator = read $ last problem
         in (numbers, operator)
   in map parse problems

calculate :: Operator -> [Int] -> Int
calculate Add = sum
calculate Multiply = product

calculateFrom :: ([Int], Operator) -> Int
calculateFrom (numbers, operator) = calculate operator numbers

part1 :: String -> Int
part1 input =
  let problems = parseInput1 input
   in problems & map calculateFrom & sum

parseInput2 :: String -> [([Int], Operator)]
parseInput2 input =
  let transposed = lines input & transpose
      withOperator :: [([Char], Maybe Operator)] = transposed & map (\line -> (init line, readMaybe $ [last line]))
      grouped = groupBy (\_ (_, operator) -> isNothing operator) withOperator
      problems = grouped & map (\group -> (map fst group, fromJust $ snd $ head group))
   in problems & map (\(numbers, operator) -> (map readMaybe numbers & catMaybes, operator))

part2 :: String -> Int
part2 input =
  let problems = parseInput2 input
   in problems & map calculateFrom & sum