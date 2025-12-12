module Days.D04 (part1, part2) where

import Data.Function ((&))
import Data.Vector (Vector)
import Data.Vector qualified as V
import Lib (parseInput)
import Text.Parsec (char, many1, newline, sepBy1, (<|>))
import Text.Parsec.String (Parser)

data Space = Free | Removed | Paper
  deriving (Eq)

instance Show Space where
  show :: Space -> String
  show Free = "."
  show Removed = "x"
  show Paper = "@"

newtype Row = Row (Vector Space)

instance Show Row where
  show :: Row -> String
  show (Row vec) = (vec & V.map (\space -> show space) & concat) ++ "\n"

newtype Department = Department (Vector Row)

fromVectors :: Vector (Vector Space) -> Department
fromVectors vecs = Department $ V.map Row vecs

instance Show Department where
  show :: Department -> String
  show (Department department) =
    "\n" ++ (department & V.map show & concat)

fromList :: [[Space]] -> Department
fromList list = Department $ V.fromList $ map (\row -> Row $ V.fromList row) list

spaceParser :: Parser Space
spaceParser =
  (char '.' >> return Free) <|> (char '@' >> return Paper)

departmentParser :: Parser Department
departmentParser = fromList <$> (many1 spaceParser) `sepBy1` newline

getAdjecent :: Int -> Int -> Department -> [Space]
getAdjecent row col (Department department) =
  let rows = V.length department
      cols = let Row vec = department V.! 0 in V.length vec
      positions = [(r, c) | r <- [row - 1 .. row + 1], c <- [col - 1 .. col + 1], (r, c) /= (row, col)]
      valid = filter (\(r, c) -> r >= 0 && r < rows && c >= 0 && c < cols) positions
   in map (\(r, c) -> let Row vec = department V.! r in vec V.! c) valid

mapWithAdjecent :: ([Space] -> Space -> a) -> Department -> Vector (Vector a)
mapWithAdjecent f (Department department) =
  V.imap
    ( \row (Row rowVec) ->
        V.imap
          ( \col cell ->
              let adjecent = getAdjecent row col (Department department)
               in f adjecent cell
          )
          rowVec
    )
    department

countType :: Space -> Department -> Int
countType space (Department department) =
  V.sum $ V.map (\(Row row) -> V.length $ V.filter (== space) row) department

anyType :: Space -> Department -> Bool
anyType space (Department department) =
  V.any (\(Row row) -> V.any (== space) row) department

takePaper :: Department -> Department
takePaper department =
  fromVectors $
    mapWithAdjecent
      ( \adjacent cell -> case cell of
          Free -> Free
          Removed -> Free
          Paper ->
            if (length $ filter (\space -> space == Paper) $ adjacent) < 4
              then Removed
              else Paper
      )
      department

part1 :: String -> Int
part1 input =
  let department = parseInput departmentParser input
   in department & takePaper & countType Removed

part2 :: String -> Int
part2 input =
  let department = parseInput departmentParser input
      paper = countType Paper department
      clearOut d =
        let cleared = takePaper d
            changed = anyType Removed cleared
         in if changed
              then clearOut cleared
              else cleared
   in paper - countType Paper (clearOut department)
