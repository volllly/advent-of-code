module Days.D05 (part1, part2) where

import Data.Function ((&))
import Data.Interval
import Data.IntervalSet (fromList, toList)
import Lib (parseInput)
import Text.Parsec
import Text.Parsec.String (Parser)

intervalParser :: Parser (Interval Int)
intervalParser = do
  from <- (read :: String -> Int) <$> many1 digit
  _ <- char '-'
  to <- (read :: String -> Int) <$> many1 digit
  return $ (Finite from <=..<= Finite to)

intervalsParser :: Parser [Interval Int]
intervalsParser = intervalParser `endBy1` newline

inputParser :: String -> ([Interval Int], [Int])
inputParser =
  parseInput
    ( do
        r <- intervalsParser
        _ <- newline
        i <- (read <$> many1 digit) `sepBy1` newline
        return (r, i)
    )

part1 :: String -> Int
part1 input =
  let (ranges, ingredients) = inputParser input
      fresh = ingredients & filter (\ingredient -> ranges & any (\range -> ingredient `member` range))
   in length fresh

part2 :: String -> Int
part2 input =
  let (ranges, _) = inputParser input
      fresh = fromList ranges
   in toList fresh & map ((+ 1) . width) & sum