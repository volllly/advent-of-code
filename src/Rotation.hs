module Rotation (Rotation (..), rotationParser) where

import Text.Parsec
import Text.Parsec.String (Parser)

data Rotation = Left | Right
  deriving (Show, Eq)

rotationParser :: Parser Rotation
rotationParser =
  (char 'L' >> return Rotation.Left) <|> (char 'R' >> return Rotation.Right)