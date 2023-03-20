fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<yew_app::App>::new().render();
}
