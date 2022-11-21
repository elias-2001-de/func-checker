module View exposing (view)

import Bootstrap.Button as Button
import Bootstrap.Dropdown as Dropdown
import Bootstrap.Grid as Grid
import Bootstrap.Grid.Col as Col
import Html exposing (Html, div, input, text)
import Html.Attributes exposing (value)
import Html.Events exposing (onInput)
import List exposing (indexedMap)
import Model exposing (..)


view : Model -> Html Msg
view model =
    div []
        [ Grid.container []
            [ Grid.row []
                [ Grid.col [] []
                , Grid.col [ Col.xs8 ] []
                , Grid.col []
                    [ Dropdown.dropdown
                        model.dropState
                        { options = []
                        , toggleMsg = ChangePrintType
                        , toggleButton =
                            Dropdown.toggle [ Button.outlinePrimary ] [ text "Print Mode" ]
                        , items =
                            [ Dropdown.buttonItem [] [ text "Markdown" ] --onClick Item1Msg
                            , Dropdown.buttonItem [] [ text "Latex" ] -- onClick Item2Msg
                            ]
                        }
                    ]
                ]
            , Grid.row []
                [ Grid.col [] []
                , Grid.col [ Col.xs8 ] [ div [] (indexedMap func2Htlm model.funcs) ]
                , Grid.col [] []
                ]
            , Grid.row []
                [ Grid.col [] []
                , Grid.col
                    [ Col.xs8 ]
                    [ div []
                        [ Button.button [ Button.outlinePrimary, Button.onClick Delete ] [ text "Delete" ]
                        , Button.button [ Button.outlinePrimary, Button.onClick Add ] [ text "Add" ]
                        , Button.button [ Button.outlinePrimary, Button.onClick Table ] [ text "Table" ]
                        , Button.button [ Button.outlinePrimary, Button.onClick (Print model.printType) ] [ text "Print" ]
                        ]
                    ]
                , Grid.col [] []
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
