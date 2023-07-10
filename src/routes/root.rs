use axum::response::Html;
use leptos::{
    view,
    IntoView,
    ssr::render_to_string,
};

use crate::components::meta::Metadata;

pub async fn root() -> Html<String> {
    let html = render_to_string(|cx| view! {
        cx,
        <Metadata />
        <div class="text-xl font-medium text-blue-600">"Hello"</div>
    });

    return Html(html);
}
