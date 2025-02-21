use ds::chart::gen_candlestick_svg;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

async fn get_chart() -> String {
    gen_candlestick_svg().await.unwrap_or_else(|_| "Failed to get Data".to_string())
}

#[component]
fn App() -> impl IntoView {
    let chart_resource = LocalResource::new(move || get_chart());

    let async_result = move || {
        chart_resource
            .get()
            .as_deref()
            .map(|value| format!("{value}"))
            .unwrap_or_else(|| "Loading...".into())
    };

    view! {
        <h1> { "Hello World!" }</h1>
        <div inner_html = async_result />
    }
}
