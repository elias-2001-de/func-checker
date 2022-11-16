port module Subscribe exposing (..)

import Model exposing (..)


port setFunc : List String -> Cmd msg


port getStatus : (List String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions _ =
    getStatus NewStatus
