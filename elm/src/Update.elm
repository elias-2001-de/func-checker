module Update exposing (update)

-- import Main exposing (setFunc)

import List exposing (append, drop, indexedMap, length, map, take)
import Model exposing (..)
import String exposing (dropRight)
import Subscribe exposing (setFunc)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangePrintType state ->
            ( { model | dropState = state }
            , Cmd.none
            )

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
            changeText model index text

        NewStatus message ->
            ( { model | funcs = updateStatus (map str2Status message) model.funcs }, Cmd.none )


changeText : Model -> Int -> String -> ( Model, Cmd Msg )
changeText model index text =
    ( { model
        | funcs = updateElement (indexedMap Tuple.pair model.funcs) index text
      }
    , setFunc
        (map getData
            (updateElement (indexedMap Tuple.pair model.funcs) index text)
        )
    )


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


updateStatus : List Status -> List Func -> List Func
updateStatus msg func =
    map updateState (zip msg func)


str2Status : String -> Status
str2Status str =
    case dropRight 5 str of
        "Wrong" ->
            Wrong

        "Corre" ->
            Correct

        _ ->
            Error str


updateState : ( Status, Func ) -> Func
updateState ( staus, func ) =
    { func | status = staus }


zip : List a -> List b -> List ( a, b )
zip xs ys =
    List.map2 Tuple.pair xs ys
