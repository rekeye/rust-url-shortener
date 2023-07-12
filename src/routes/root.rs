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
        <main class="min-h-screen bg-neutral-100 flex flex-col gap-4 items-center">
            <img class="w-48 m-4 self-start" src="/assets/logo.webp" alt="LinkEase" />
            <div class="mx-4 max-w-4xl">
                <section class="flex flex-col gap-4">
                    <h1 class="font-medium text-2xl text-left">"Introducing LinkEase: Simplify Your URLs with Ease"</h1>
                    <p>
                        r##"
                        Long, complicated URLs are a thing of the past.
                        Say goodbye to tangled web addresses that are difficult to remember and share. 
                        Welcome to LinkEase, your trusted URL shortening service that effortlessly streamlines your links.
                        "##
                    </p>
                    <p>
                        r##"
                        With LinkEase, you can transform any lengthy URL into a concise, memorable, and clickable link.
                        Whether you're a marketer, social media enthusiast, or simply someone who wants to make their online presence more efficient, 
                        LinkEase is here to help you simplify and amplify.
                        "##
                    </p>
                </section>
            </div>
            <div class="w-full p-4 bg-neutral-50 shadow flex justify-center">
                <form class="max-w-4xl w-full flex flex-col gap-4 mb-0"
                    hx-post="/api/create-hash"
                    hx-ext="json-enc" 
                    hx-swap="outerHTML" >
                    <label class="text-xl font-medium" for="input_url">
                        "Paste your long URL here:"
                    </label>
                    <input class="px-6 py-4 rounded-lg shadow"
                        type="url"
                        name="input_url"
                        placeholder="https://example.com"
                        pattern="https://.*"
                        size="30"
                        required />
                    <button class="px-6 py-4 rounded-lg shadow bg-primary text-white font-medium">
                        "Give me the short url"
                    </button>
                </form>
            </div>
            <div class="mx-4 max-w-4xl">
                <section class="flex flex-col gap-4">
                    <h2 class="font-medium text-xl text-left">"Why Choose LinkEase?"</h2>
                    <p>
                        r##"
                        Don't let lengthy URLs hold you back. 
                        Embrace the power of simplicity with LinkEase. 
                        Your online journey just got shorter, smarter, and more impactful.
                        "##
                    </p>
                </section>
            </div>
        </main>
    });

    return Html(html);
}

