use axum::response::Html;
use leptos::{
    view,
    component,
    IntoView,
    Scope,
    ssr::render_to_string,
};

use crate::components::meta::Metadata;

pub async fn root() -> Html<String> {
    let html = render_to_string(|cx| view! {
        cx,
        <Metadata />
        <main class="min-h-screen p-4 bg-neutral-600 flex flex-col gap-4 items-center justify-center">
            <UrlForm />
        </main>
    });

    return Html(html);
}

#[component]
fn UrlForm(cx: Scope) -> impl IntoView {
    return view! {
        cx,
        <form class="flex flex-col gap-4"
            hx-post="/api/create-hash"
            hx-ext="json-enc"
            hx-target="#shortened-url" >
            <input class="px-6 py-2 rounded-lg shadow"
                type="url"
                name="input_url"
                placeholder="https://example.com"
                pattern="https://.*"
                size="30"
                required />
            <button class="px-6 py-2 rounded-lg shadow bg-emerald-800 text-white font-medium">"Give me the short url"</button>
            <div id="shortened-url"></div>
        </form>
    }
}
