module Days.D02 (part1, part2) where

import Data.Function ((&))
import Data.List (List, scanl', unfoldr)
import Debug.Trace (traceShow)
import Text.Parsec
import Text.Parsec.String (Parser)

data Range a = Range
  { from :: a,
    to :: a
  }
  deriving (Show)

rangeParser :: Parser (Range Int)
rangeParser = do
  from <- read <$> many1 digit
  _ <- char '-'
  to <- read <$> many1 digit
  return $ Range from to

rangesParser :: Parser (List (Range Int))
rangesParser = do
  rangeParser `sepBy1` char ','

parseInput :: String -> Either ParseError (List (Range Int))
parseInput = parse rangesParser ""

chunksOf :: Int -> [a] -> [[a]]
chunksOf n = unfoldr (\xs -> if null xs then Nothing else Just (splitAt n xs))

divisors :: Int -> [Int]
divisors n = [d | d <- [1 .. n], n `mod` d == 0]

allEqualSplits :: (Int -> [Int]) -> [a] -> [[[a]]]
allEqualSplits divisors xs = [chunksOf d xs | d <- divisors (length xs)]

part :: (Int -> [Int]) -> String -> Int
part divisors input =
  parseInput input
    & either (error . show) id
    >>= (\r -> [from r .. to r])
    & filter
      ( \id ->
          let splits = allEqualSplits divisors (show $ id)
           in if null splits then False else splits & any (\split -> split & all (== head split))
      )
    & sum

part1 :: String -> Int
part1 = part (\n -> if even n then [n `div` 2] else [])

part2 :: String -> Int
part2 = part (\n -> divisors n & filter (\d -> d /= n))
