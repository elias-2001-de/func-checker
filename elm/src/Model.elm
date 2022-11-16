module Model exposing (..)


type alias Model =
    { funcs : List Func, printType : PrintType }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { funcs =
            [ { data = "a&b|c", status = Correct }
            , { data = "a&b|c", status = Correct }
            ]
      , printType = Markdown
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


type Status
    = Correct
    | Wrong
    | Error String


type PrintType
    = Latex
    | Markdown


type alias Func =
    { data : String, status : Status }
