mod sim;
mod vec;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|| "Hello, world!".into());
}
