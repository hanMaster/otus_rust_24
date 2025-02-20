use leptos::html::HtmlElement;
use ds::chart::gen_candlestick_svg;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

async fn get_chart() -> String {
    match gen_candlestick_svg().await {
        Ok(data) => {data}
        Err(_) => {"Failed to get Data".to_string()}
    }
}

#[component]
fn App() -> impl IntoView {
    let once = LocalResource::new(move || get_chart());
    let svg = move || Suspend::new(async move { once.await });

    view! {
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <div>
                { svg }
            </div>
        </Suspense>
    }
}