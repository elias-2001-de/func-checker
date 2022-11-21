module Model exposing (..)

import Bootstrap.Dropdown as Dropdown


type alias Model =
    { funcs : List Func, printType : PrintType, dropState : Dropdown.State }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { funcs =
            [ { data = "a&b|c", status = Correct }
            , { data = "a&b|c", status = Correct }
            ]
      , printType = Markdown
      , dropState = Dropdown.initialState
      }
    , Cmd.none
    )


type Msg
    = Delete
    | Add
    | Table
    | Print PrintType
    | ChangeText Int String
    | NewStatus (List String)
    | ChangePrintType Dropdown.State


type Status
    = Correct
    | Wrong
    | Error String


type PrintType
    = Latex
    | Markdown


type alias Func =
    { data : String, status : Status }
