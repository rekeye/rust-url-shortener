use leptos::{
    view,
    IntoView,
    Scope, 
    component,
};

#[component]
pub fn Metadata(cx: Scope) -> impl IntoView {
    return view! {
        cx,
        <html lang="en">
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"LinkEase | Simplify Your URLs with Ease"</title>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
            <link href="/styles.css" rel="stylesheet" />
            <link rel="icon" type="image/x-icon" href="/assets/favicon.ico" />
        </html>
    }
}
