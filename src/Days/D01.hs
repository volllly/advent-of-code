module Days.D01 (part1, part2) where

import Data.Function ((&))
import Data.List (scanl')
import Rotation
import Text.Parsec
import Text.Parsec.String (Parser)

data Instruction = Instruction
  { rotation :: Rotation,
    amount :: Int
  }
  deriving (Show)

instructionParser :: Parser Instruction
instructionParser = do
  rot <- rotationParser
  amt <- read <$> many1 digit
  return $ Instruction rot amt

parseInstruction :: String -> Either ParseError Instruction
parseInstruction = parse instructionParser ""

part1 :: String -> Int
part1 input =
  lines input
    & mapM parseInstruction
    & either (error . show) id
    & scanl'
      ( \acc i ->
          ( acc + case rotation i of
              Rotation.Left -> -(amount i)
              Rotation.Right -> amount i
          )
            `mod` 100
      )
      50
    & filter (== 0)
    & length

part2 :: String -> Int
part2 input =
  lines input
    & mapM parseInstruction
    & either (error . show) id
    & foldl'
      ( \(currentPosition, zeroes) i ->
          let steps =
                amount i * case rotation i of
                  Rotation.Left -> -1
                  Rotation.Right -> 1
              (rotations, newPosition) = (currentPosition + steps) `divMod` 100
              zeroStart = if currentPosition == 0 then 1 else 0
              zeroEnd = if newPosition == 0 then 1 else 0
              additionalZeroes =
                if steps >= 0
                  then rotations
                  else abs (rotations + zeroStart - zeroEnd)
           in (newPosition, zeroes + additionalZeroes)
      )
      (50, 0)
    & snd