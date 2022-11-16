module View exposing (..)

import Html exposing (Html, button, div, input, text)
import Html.Attributes exposing (value)
import Html.Events exposing (onClick, onInput)
import List exposing (indexedMap)
import Model exposing (..)


view : Model -> Html Msg
view model =
    div []
        [ -- div [] [ select [ style "font-size" "14px" ] [ text "Latex", text "Markdown" ] ] ,
          div []
            [ div [] (indexedMap func2Htlm model.funcs)
            , div
                []
                [ button [ onClick Delete ] [ text "Delete" ]
                , button [ onClick Add ] [ text "Add" ]
                , button [ onClick Table ] [ text "Table" ]
                , button [ onClick (Print model.printType) ] [ text "Print" ]
                ]
            ]
        ]


func2Htlm : Int -> Func -> Html Msg
func2Htlm index func =
    div []
        [ input
            [ value func.data
            , onInput (ChangeText index)
            ]
            []
        , showStaus func.status
        ]


showStaus : Status -> Html Msg
showStaus status =
    case status of
        Correct ->
            --i [ class "bi bi-check-circle-fill text-success" ] []
            text "Correct"

        Wrong ->
            -- i [ class "bi bi-x-circle-fill text-danger" ] []
            text "Wrong  "

        Error msg ->
            -- i [ class "bi bi-exclamation-diamond-fill text-warning" ] []
            text msg
