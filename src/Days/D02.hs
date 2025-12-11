{-# OPTIONS_GHC -Wno-name-shadowing #-}

module Days.D02 (part1, part2) where

import Data.List.Split (chunksOf)
import Text.Parsec
import Text.Parsec.String (Parser)

data Range = Range
  { rangeFrom :: Int,
    rangeTo :: Int
  }
  deriving (Show)

rangeParser :: Parser Range
rangeParser = do
  from <- read <$> many1 digit
  _ <- char '-'
  to <- read <$> many1 digit
  return $ Range from to

rangesParser :: Parser [Range]
rangesParser = rangeParser `sepBy1` char ','

parseInput :: String -> Either ParseError [Range]
parseInput = parse rangesParser ""

allEqualSplits :: (Int -> [Int]) -> [a] -> [[[a]]]
allEqualSplits divisors xs = [chunksOf d xs | d <- divisors (length xs)]

allChunksEqual :: (Eq a) => [a] -> Bool
allChunksEqual [] = True
allChunksEqual (x : xs) = all (== x) xs

solve :: (Int -> [Int]) -> String -> Int
solve divisors input =
  let ranges = either (error . show) id $ parseInput input
      nums = concatMap (\r -> [rangeFrom r .. rangeTo r]) ranges
      isValid n = any allChunksEqual $ allEqualSplits divisors $ show n
   in sum $ filter isValid nums

part1 :: String -> Int
part1 = solve (\n -> [n `div` 2 | n > 1, even n])

part2 :: String -> Int
part2 = solve (\n -> [d | d <- [1 .. n - 1], n `mod` d == 0])
