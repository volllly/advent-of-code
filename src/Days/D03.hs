module Days.D03 (part1, part2) where

import Data.Function (on, (&))
import Data.List (maximumBy)
import Lib (parseInput)
import Text.Parsec (many1, newline, sepBy1)
import Text.Parsec qualified (digit)
import Text.Parsec.String (Parser)

bankParser :: Parser [Word]
bankParser = many1 ((read . pure) <$> Text.Parsec.digit)

banksParser :: Parser [[Word]]
banksParser = bankParser `sepBy1` newline

largestDigit :: [Word] -> (Int, Word)
largestDigit bank = maximumBy (compare `on` snd) (reverse (zip [0 ..] bank))

digits :: Int -> [Word] -> [Word]
digits 0 _ = []
digits places bank =
  let possible = take (length bank - places + 1) bank
      digit = largestDigit possible
      remaining = drop (1 + fst digit) bank
   in [snd digit] ++ digits (pred places) remaining

part :: Int -> String -> Int
part places input =
  let banks = parseInput banksParser input
      joltDigits bank = digits places bank
      jolt bank = joltDigits bank & foldl (\acc d -> acc * 10 + d) 0
   in fromIntegral $ sum $ map jolt banks

part1 :: String -> Int
part1 = part 2

part2 :: String -> Int
part2 = part 12
