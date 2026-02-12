{-# LANGUAGE OverloadedStrings #-}
module Main where

import Network.Wai (responseLBS, Application, pathInfo)
import Network.Wai.Handler.Warp (run)
import Network.HTTP.Types (status200, status404)
import qualified Data.ByteString.Lazy.Char8 as LBS

app :: Application
app req respond = case pathInfo req of
  ["health"] -> respond $ responseLBS status200 [] (LBS.pack "ok")
  _          -> respond $ responseLBS status404 [] (LBS.pack "not found")

main :: IO ()
main = do
  putStrLn "haskell-api listening on :8003"
  run 8003 app
