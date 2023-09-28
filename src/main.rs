use psychic_octo_potato::app::App;
use yew::prelude::*;
    fn main() {
     wasm_logger::init(wasm_logger::Config::default());
        yew::Renderer::<App>::new().render();
}
