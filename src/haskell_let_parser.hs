
parseWhole :: [Char] -> Either ParseError String
parseWhole = runParser getScoped S "WGSL"
  where
    getScoped = do
      choice [try readAll, return "error"]

readAll :: Par String
readAll = do manyTill anyChar eof

-- -- finds "let" tokens and return their position so that we can raplace them with vars if applicable
-- findLetParse :: [Char] -> Either ParseError [(Position, Name)]
-- findLetParse =
--   runParser manyLets W "WGSL"

-- finds "let" tokens and return their position so that we can raplace them with vars if applicable
-- findLetParse :: [Char] -> Either ParseError [(Position, Name)]
findLetParse :: [Char] -> Either ParseError String
findLetParse =
  runParser manyLets S "WGSL"

-- manyLets :: Par [(Position, Name)]
manyLets :: Par String
manyLets = fmap join $ do manyTill (try checkLets) eof

-- oneLet :: Par (Position, Name)
-- oneLet = do
--   choice [letPos, readAll >> return (-1, "err")]

-- [letPos, return (-1, "")]

-- letPos :: Par (Position, Name)
-- letPos = do
--   void $ (tilKeyword "let")
--   intpos <- getLineNumber . show <$> getPosition
--   return (intpos, "let")

-- keywordLet :: GenParser Char st ()
-- keywordLet =
--   try
--     ( do
--         _ <- string "let"
--         notFollowedBy alphaNum
--     )

-- IT WORKS:
-- 0. return let positions that have updates
-- 1. make sure the update of type p.xy are detected
-- 2. replace lets

-- either return LET name = ...
-- or            VAR name = ...
-- either return ___ name: type = ...
-- or            ___ name = ...

checkLets :: Par String
checkLets = do
  choice
    [ try $ do
        meh <- manyTill anyChar $ try $ string "let "
        -- void $ try (string "let ")

        name <- manyTill anyChar $ lookAhead typeOrSpace
        ts <- typeOrSpace
        -- c <- anyChar

        -- ts <- typeOrSpace
        -- typeOrSpace <-

        maybeName <- lookAhead $ isRepeated name
        if maybeName
          then return (meh ++ "var " ++ name ++ ts)
          else return (meh ++ "let " ++ name ++ ts),
      manyTill anyChar eof >> return ""
    ]

typeOrSpace :: Par String
typeOrSpace = do
  choice [try readType, string " "]

-- letOrVar :: Par String
-- letOrVar = do
--   option "let " $
--     (try $ do
--         name <- manyTill anyChar (choice [try readType, string " "])

--     )

readType :: Par String
readType = do
  c <- string ": "
  t <- manyTill anyChar (char ' ')
  return $ ": " ++ t ++ " "

-- finds whether or not a declared variable is being updated later in the file
isRepeated :: String -> Par Bool
isRepeated name = do
  option
    False
    $ do
      void $ try $ lookAhead $ manyany $ try $ string (name ++ " = ")
      return True