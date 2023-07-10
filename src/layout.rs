use leptos::*;

#[component]
pub fn Layout(
    cx: Scope,
    children: Children
) -> impl IntoView {
    return view! {
        cx,
        <html>
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            </head>
            <body>
                {children(cx)
                    .nodes
                    .into_iter()
                    .map(|child| view! { cx, {child} })
                    .collect::<Vec<_>>()}
            </body>
        </html>
    }
}
