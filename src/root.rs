use axum::response::Html;
use leptos::*;

use crate::layout::*;

pub async fn root() -> Html<String> {
    let html = ssr::render_to_string(|cx| view! {
        cx,
        <Layout>
            <div>"Hello"</div>
        </Layout>
    });

    return Html(html);
}
