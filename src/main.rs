mod components;
mod logic;
mod pages;

fn main() {
    yew::start_app::<pages::BoolAlgebra>();
    wasm_logger::init(wasm_logger::Config::default());
}
