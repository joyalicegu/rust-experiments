fn main() {
    let css = r#"body {
        background-color: #000;
        margin: 0;
        overflow: hidden;
    }
    html {
        touch-action: manipulation;
    }"#;

    cargo_run_wasm::run_wasm_with_css(css);
}
