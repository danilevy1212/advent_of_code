-- |
module Main where

import Data.List.Split (splitOn)
import Data.Char (toLower, isAlpha)
import Data.Maybe (mapMaybe)
import Text.Read (readMaybe)

type Tokens = [String]

data Password = Password
  { charMin :: Int,
    charMax :: Int,
    character :: Char,
    text :: String
  }
  deriving (Show)

main :: IO ()
main = do
  input <- getInput
  print $ length $ mapMaybe toPassword input

getInput :: IO [String]
getInput = do
  putStrLn "Input path: "
  path <- getLine
  inputContents <- readFile path
  return $ lines inputContents

toPassword :: String -> Maybe Password
toPassword line
  | length tokens == 3 = maybePassword range charRest password
  | otherwise = Nothing
    where
      tokens = words line
      range = getRange $ head tokens
      charRest = getCharRestriction $ tokens !! 1
      password = last tokens

maybePassword :: Maybe (Int, Int) -> Maybe Char -> String -> Maybe Password
maybePassword (Just (minChar, maxChar)) (Just char) pass =
  if isValidPass then Just passRecord else Nothing
  where
    passRecord = Password
          { charMin = minChar,
            charMax = maxChar,
            character = char,
            text = pass
          }
    isValidPass = isPasswordValid passRecord
maybePassword _ _ _ = Nothing

getRange :: String -> Maybe (Int, Int)
getRange a
  | length ranges == 2 = maybeFirstEqOrSmallerThan (maxChar, minChar)
  | otherwise = Nothing
  where
    ranges = splitOn "-" a
    intRanges = sequence $ readMaybe <$> ranges :: Maybe [Int]
    maxChar = head <$> intRanges
    minChar = last <$> intRanges

maybeFirstEqOrSmallerThan :: Ord a => (Maybe a, Maybe a) -> Maybe (a, a)
maybeFirstEqOrSmallerThan (Just x, Just y)
  | x <= y = Just (x, y)
  | otherwise = Nothing
maybeFirstEqOrSmallerThan _ = Nothing

getCharRestriction :: String -> Maybe Char
getCharRestriction "" = Nothing
getCharRestriction a
  | isAlpha firstChar = Just $ toLower firstChar
  | otherwise = Nothing
    where
      firstChar = head a

-- Part 1
-- isPasswordValid :: Password -> Bool
-- isPasswordValid password = isBetween
--     where
--       times = length $ filter (== character password) (text password)
--       isBetween = times >= charMin password && times <= charMax password

-- Part 2
isPasswordValid :: Password -> Bool
isPasswordValid password = xOr isInMax isInMin
    where
      pass = text password
      charR = character password
      isInMin = (pass !! (charMin password - 1)) == charR
      isInMax = (pass !! (charMax password - 1)) == charR

xOr :: Bool -> Bool -> Bool
xOr True True = False
xOr False False = False
xOr _ _ = True
