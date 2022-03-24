use crate::components::InputFunc;
use yew::html::ComponentLink;
use yew::prelude::*;
use yew::{html, Component, Html};

pub enum Msg {
    // RemoveFunc(usize),
}

#[derive(Debug)]
pub struct BoolAlgebra {
    link: ComponentLink<Self>,
}

impl Component for BoolAlgebra {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>

                <h1>{ "function checker" }</h1>
                <InputFunc func="a&b|c"/>
            </main>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {}
        false
    }
}
