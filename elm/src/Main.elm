module Main exposing (..)

-- import Subscribe exposing (subscriptions)

import Browser
import Model exposing (..)
import Subscribe exposing (subscriptions)
import Update exposing (update)
import View exposing (view)


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
