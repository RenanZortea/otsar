use leptos::*;
use shared::parse_custom_syntax;

#[component]
pub fn NoteEditor() -> impl IntoView {
    // 1. Signal to hold raw user text
    let (text, set_text) =
        create_signal("Try typing: Hello $(World, text-3xl text-red-500 font-bold)".to_string());

    // 2. Derived signal that parses text automatically when it changes
    let parsed_html = create_memo(move |_| parse_custom_syntax(&text.get()));

    view! {
        <div class="grid gap-6">
            // INPUT AREA
            <div class="flex flex-col gap-2">
                <label class="font-semibold text-sm uppercase tracking-wider">"Editor"</label>
                <textarea
                    class="w-full h-32 p-4 rounded-lg border border-gray-300 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all shadow-sm font-mono text-sm"
                    prop:value=text
                    on:input=move |ev| set_text.set(event_target_value(&ev))
                />
            </div>

            // PREVIEW AREA
            <div class="flex flex-col gap-2">
                <label class="font-semibold text-sm uppercase tracking-wider">"Preview (Rendered HTML)"</label>

                // We use inner_html to render the parsed span tags
                <div
                    class="min-h-32 p-6 rounded-lg bg-white shadow-md border border-gray-100 prose max-w-none"
                    inner_html=parsed_html
                />
            </div>

            // SYNTAX HELP
            <div class="bg-blue-50 text-blue-800 p-4 rounded text-sm">
                <strong>"Tip: "</strong>
                "Use syntax "
                <code class="bg-blue-100 px-1 rounded">"$(Content, tailwind-classes)"</code>
                " to style text."
            </div>
        </div>
    }
}
