use crate::components::note_editor::NoteEditor;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 p-8 font-sans text-gray-800">
            <h1 class="text-4xl font-bold mb-8 text-center text-indigo-600">
                "Super Note App"
            </h1>

            <div class="max-w-2xl mx-auto">
                <NoteEditor />
            </div>

            <div class="mt-8 text-center text-sm text-gray-400">
                "Powered by Tauri, Leptos & UnoCSS"
            </div>
        </div>
    }
}
