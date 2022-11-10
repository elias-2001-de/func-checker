port module Main exposing (..)

import Browser
import Html exposing (Html, button, div, input, text)
import Html.Attributes exposing (value)
import Html.Events exposing (onClick, onInput)
import List exposing (append, drop, indexedMap, length, map, take)



-- MAIN


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }



-- PORTS


port sendMessage : List String -> Cmd msg


port messageReceiver : (List String -> msg) -> Sub msg



-- MODEL


type alias Model =
    { funcs : List Func }


type alias Func =
    { data : String, status : Status }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { funcs =
            [ { data = "a&b|c", status = Correct }
            , { data = "a&b|c", status = Correct }
            ]
      }
    , Cmd.none
    )



-- UPDATE


type Msg
    = Delete
    | Add
    | Table
    | Print PrintType
    | ChangeText Int String
    | Recv (List String)


type Status
    = Correct
    | Wrong
    | Error String


type PrintType
    = Latex
    | Markdown


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Delete ->
            ( { model
                | funcs =
                    if length model.funcs == 1 then
                        model.funcs

                    else
                        take (length model.funcs - 1) model.funcs
              }
            , Cmd.none
            )

        Add ->
            ( { model
                | funcs =
                    if length model.funcs == 1 then
                        append model.funcs model.funcs

                    else
                        append model.funcs (drop (length model.funcs - 1) model.funcs)
              }
            , Cmd.none
            )

        Table ->
            ( model, Cmd.none )

        Print _ ->
            ( model, Cmd.none )

        ChangeText index text ->
            ( { model | funcs = updateElement (indexedMap Tuple.pair model.funcs) index text }
            , sendMessage (map getData (updateElement (indexedMap Tuple.pair model.funcs) index text))
            )

        Recv message ->
            ( model, Cmd.none )


getData : Func -> String
getData f =
    f.data


updateElement : List ( Int, Func ) -> Int -> String -> List Func
updateElement list id text =
    let
        toggle ( idx, func ) =
            if id == idx then
                { func | data = text }

            else
                func
    in
    map toggle list


subscriptions : Model -> Sub Msg
subscriptions _ =
    messageReceiver Recv



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ -- div [] [ select [] [ text "Latex", text "Markdown" ] ],
          div []
            [ div [] (indexedMap func2Htlm model.funcs)
            , div
                []
                [ button [ onClick Delete ] [ text "Delete" ]
                , button [ onClick Add ] [ text "Add" ]
                , button [ onClick Table ] [ text "Table" ]
                , button [ onClick (Print Markdown) ] [ text "Print" ]
                ]
            ]
        ]


func2Htlm : Int -> Func -> Html Msg
func2Htlm index func =
    div [] [ input [ value func.data, onInput (ChangeText index) ] [], showStaus func.status ]


showStaus : Status -> Html Msg
showStaus status =
    case status of
        Correct ->
            text "Correct"

        Wrong ->
            text "Wrong"

        Error msg ->
            text msg
