module Lib (parseInput, inspect) where

import Debug.Trace (traceShow)
import Text.Parsec (parse)
import Text.Parsec.String (Parser)

parseInput :: Parser a -> String -> a
parseInput parser input = either (error . show) id $ parse parser "" input

inspect :: (Show t) => t -> t
inspect t = traceShow t t