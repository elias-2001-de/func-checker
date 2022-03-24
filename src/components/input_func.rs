use crate::logic::{latex_func, latex_table, lex};
use bool_algebra::{get_names, parse, Token};
use yew::html::ComponentLink;
use yew::prelude::*;
use yew::{html, Component, Html, InputData};

#[derive(Clone, Debug)]
pub enum Msg {
    OnTextInput(String),
    UpdateState,
    RemoveSelf,
    CreateNew,
    PrintTable,
    PrintLatex,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::new())]
    pub func: String,
    #[prop_or(vec!['&', '|', '^', '!', '>', '<', '='])]
    pub operators: Vec<char>,
    #[prop_or(None)]
    pre_result: Option<Vec<bool>>,
    #[prop_or(None)]
    pre_var_names: Option<Vec<String>>,
    #[prop_or(State::Correct)]
    pre_state: State,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Correct,
    Wrong,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct InputFunc {
    func: String,
    state: State,
    link: ComponentLink<Self>,
    props: Props,
    result: Option<Vec<bool>>,
    has_child: bool,
    var_names: Option<Vec<String>>,
}

impl Component for InputFunc {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut var_names = None;
        let (state, result) = if props.func.is_empty() {
            (State::Error("no function".to_string()), None)
        } else {
            let func_res = lex(&props.func.clone(), &props.operators.clone());
            if let Ok(func) = func_res.clone() {
                var_names = Some(bool_algebra::get_names(&func));
            }
            Self::update_state(func_res, props.pre_result.clone())
        };
        Self {
            var_names,
            state,
            props: props.clone(),
            func: props.func,
            link,
            result,
            has_child: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.operators != props.operators {
            self.props.operators = props.operators;
        }
        if self.props.pre_result != props.pre_result {
            self.props.pre_result = props.pre_result;
        }
        if self.props.pre_state != props.pre_state {
            self.props.pre_state = props.pre_state;
        }
        true
    }

    fn view(&self) -> Html {
        let state = match self.state {
            State::Correct => html! {<i class={"bi bi-check-circle-fill text-success"}></i>},
            State::Wrong => html! {<i class={"bi bi-x-circle-fill text-danger"}></i>},
            State::Error(_) => {
                html! {<i class={"bi bi-exclamation-diamond-fill text-warning"}></i>}
            }
        };

        let btn_col = "btn btn-outline-info";
        html! {
            <div class={"bool_func"}>
                <input type="text" value={self.func.clone()}  oninput=self.link.callback(|e:InputData| Msg::OnTextInput(e.value)) />
                {state}
            {if self.has_child {
                html! { <InputFunc func=self.func.clone() operators=self.props.operators.clone() pre_result=self.result.clone() /> }
            }else{
                html! {
                    <div>
                        <button type="button" onclick=self.link.callback(|_| Msg::RemoveSelf) class={btn_col}><i class={"bi bi-trash2-fill"}></i></button>
                        <button type="button" onclick=self.link.callback(|_| Msg::CreateNew) class={btn_col}><i class={"bi bi-plus-circle-fill"}></i></button>
                        <button type="button" onclick=self.link.callback(|_| Msg::PrintTable) class={btn_col}><i class={"bi bi-table"}></i></button>
                        <button type="button" onclick=self.link.callback(|_| Msg::PrintLatex) class={btn_col}><i class={"bi bi-file-earmark-text-fill"}></i></button>
                    </div>
                }
            }}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnTextInput(value) => {
                self.func = value;
                self.update(Msg::UpdateState)
            }

            Msg::UpdateState => {
                let update = Self::update_state(self.get_tokens(), self.props.pre_result.clone());
                self.state = update.0;
                self.result = update.1;
                if self.props.pre_state != State::Correct {
                    self.state = State::Wrong;
                }
                if self.has_child {
                    // self.link.
                }
                true
            }
            Msg::RemoveSelf => {
                // self.next_node = ;
                log::info!("RemoveSelf");
                false
            }
            Msg::CreateNew => {
                self.has_child = true;
                true
            }
            Msg::PrintLatex => {
                if self.state == State::Correct {
                    let toknes = self.get_tokens().unwrap();
                    let names = get_names(&toknes);
                    let tabel_str =
                        latex_table(&self.result.clone().unwrap(), &names, &latex_func(&toknes));
                    log::info!("PrintLatex \n{}", tabel_str);
                    true
                } else {
                    false
                }
            }
            Msg::PrintTable => {
                if self.state == State::Correct || self.state == State::Wrong {
                    if let Ok(toknes) = self.get_tokens() {
                        let names = get_names(&toknes);
                        log::info!("{:?}", self.props.pre_result);
                        if let Some(table) = self.result.clone() {
                            let tabel_str = bool_algebra::print_tabel(&table, &names, &self.func);
                            log::info!("PrintTable \n{}", tabel_str);
                            return true;
                        }
                    }
                }
                false
            }
        }
    }
}

impl InputFunc {
    fn get_tokens(&self) -> Result<Vec<Token>, String> {
        lex(&self.func.clone(), &self.props.operators.clone())
    }

    fn update_state(
        token_res: Result<Vec<Token>, String>,
        result: Option<Vec<bool>>,
    ) -> (State, Option<Vec<bool>>) {
        match token_res {
            Err(e) => (State::Error(e), result),
            Ok(tokens) => match parse(&tokens) {
                Err(e) => (State::Error(e), None),
                Ok(table) => {
                    if let Some(pre_result) = result {
                        if pre_result == table {
                            (State::Correct, Some(table.clone()))
                        } else {
                            (State::Wrong, Some(table.clone()))
                        }
                    } else {
                        (State::Correct, Some(table.clone()))
                    }
                }
            },
        }
    }
}
