// src/app.rs
use crate::components::parser::ParsedText;
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Basic state for notes
    let (notes, set_notes) = create_signal(vec![Note {
        id: 1,
        title: "Demo Note".to_string(),
        content: "Welcome! Try this syntax:\n$(Big Red Text, text-3xl text-red-600 font-bold)"
            .to_string(),
    }]);

    let (active_note_id, set_active_note_id) = create_signal(1);

    // Derived signal for the currently selected note
    let active_note = move || {
        notes.with(|n| {
            n.iter()
                .find(|note| note.id == active_note_id.get())
                .cloned()
        })
    };

    // Handler to update note content
    let update_content = move |new_content: String| {
        set_notes.update(|n| {
            if let Some(note) = n.iter_mut().find(|note| note.id == active_note_id.get()) {
                note.content = new_content;
            }
        });
    };

    view! {
        <main class="container mx-auto h-screen flex overflow-hidden">
            // Sidebar
            <div class="w-1/4 bg-gray-100 border-r border-gray-300 p-4 flex flex-col gap-2">
                <h2 class="text-xl font-bold mb-4">"My Notes"</h2>
                <button
                    class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition"
                    on:click=move |_| {
                        let new_id = notes.with(|n| n.len() + 1);
                        set_notes.update(|n| n.push(Note {
                            id: new_id,
                            title: format!("Note {}", new_id),
                            content: "".to_string()
                        }));
                        set_active_note_id.set(new_id);
                    }
                >
                    "+ New Note"
                </button>

                <div class="flex flex-col gap-1 overflow-y-auto">
                    {move || notes.get().into_iter().map(|note| {
                        let is_active = note.id == active_note_id.get();
                        let bg_class = if is_active { "bg-white shadow" } else { "hover:bg-gray-200" };
                        view! {
                            <div
                                class=format!("p-3 rounded cursor-pointer transition {}", bg_class)
                                on:click=move |_| set_active_note_id.set(note.id)
                            >
                                {note.title}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Main Editor Area
            <div class="w-3/4 flex flex-col">
                {move || match active_note() {
                    Some(note) => view! {
                        <div class="flex flex-col h-full">
                            // Toolbar / Header
                            <div class="p-4 border-b border-gray-200 bg-white">
                                <h1 class="text-2xl font-bold">{note.title}</h1>
                            </div>

                            // Editor Split View
                            <div class="flex-1 flex overflow-hidden">
                                // Input Area
                                <textarea
                                    class="w-1/2 h-full p-4 resize-none border-r border-gray-200 focus:outline-none font-mono text-sm"
                                    prop:value=note.content.clone()
                                    on:input=move |ev| update_content(event_target_value(&ev))
                                    placeholder="Type here... Use $(text, classes) for styling."
                                ></textarea>

                                // Live Preview
                                <div class="w-1/2 h-full p-4 overflow-y-auto bg-gray-50">
                                    // We create a specific signal for just the content to pass to the parser
                                    <ParsedText text=create_signal(note.content).0 />
                                </div>
                            </div>
                        </div>
                    }.into_view(),
                    None => view! {
                        <div class="flex items-center justify-center h-full text-gray-400">
                            "Select a note to begin"
                        </div>
                    }.into_view()
                }}
            </div>
        </main>
    }
}
