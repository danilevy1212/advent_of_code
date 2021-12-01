-- |
module Main where

import Data.List.Split (splitOn)
import Data.Char (toLower, isAlpha)
import Data.Maybe (mapMaybe, fromJust)
import Text.Read (readMaybe)

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
  | length tokens == 3 = case isValid of
      Just True -> record
      _ -> Nothing
  | otherwise = Nothing
    where
      tokens = words line
      record = do
        range <- getRange $ head tokens
        charRest <- getCharRestriction $ tokens !! 1
        password <- Just $ last tokens
        Just Password {
          charMin = fst range,
          charMax = snd range,
          character = charRest,
          text = password
        }
      isValid = isPasswordValid <$> record

getRange :: String -> Maybe (Int, Int)
getRange a
  | length ranges == 2 = case isSmallerOrEqthan of
      (Just True) -> Just (fromJust minChar, fromJust maxChar)
      _ -> Nothing
  | otherwise = Nothing
  where
    ranges = splitOn "-" a
    intRanges = sequence $ readMaybe <$> ranges :: Maybe [Int]
    minChar = head <$> intRanges
    maxChar = last <$> intRanges
    isSmallerOrEqthan = (<=) <$> minChar <*> maxChar

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
