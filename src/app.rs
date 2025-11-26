use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GraphStats {
    node_count: usize,
    path: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Signal for the current active graph
    let (graph, set_graph) = create_signal(None::<GraphStats>);

    // Action to open folder (bridges to Tauri)
    let open_folder = create_action(move |_| async move {
        // We use serde-wasm-bindgen to pass args if needed
        let args = serde_wasm_bindgen::to_value(&()).unwrap();

        // Call the Tauri command "open_graph"
        match invoke("open_graph", args).await {
            Ok(js_val) => {
                let stats: GraphStats = serde_wasm_bindgen::from_value(js_val).unwrap();
                set_graph.set(Some(stats));
            }
            Err(e) => logging::error!("Error opening graph: {:?}", e),
        }
    });

    view! {
        <div class="h-screen w-screen bg-gray-900 text-gray-100 flex flex-col">
            // Header / Toolbar
            <div class="p-4 border-b border-gray-800 flex justify-between items-center bg-gray-900 sticky top-0 z-10">
                <h1 class="font-bold text-xl">"Logseq-rs (Leptos)"</h1>
                <button
                    class="bg-indigo-600 hover:bg-indigo-700 px-4 py-2 rounded text-sm font-medium transition-colors"
                    on:click=move |_| open_folder.dispatch(())
                >
                    {move || if open_folder.pending().get() { "Scanning..." } else { "Open Graph" }}
                </button>
            </div>

            // Main Content Area
            <div class="flex-1 overflow-auto p-4">
                {move || match graph.get() {
                    None => view! {
                        <div class="h-full flex items-center justify-center text-gray-500">
                            "No graph open. Click 'Open Graph' to start."
                        </div>
                    }.into_view(),
                    Some(stats) => view! {
                        <div class="max-w-3xl mx-auto">
                            <div class="mb-6 p-4 bg-gray-800 rounded-lg border border-gray-700">
                                <h2 class="text-lg font-semibold text-indigo-400">"Graph Loaded"</h2>
                                <p>"Path: " <span class="font-mono text-sm">{stats.path}</span></p>
                                <p>"Nodes: " {stats.node_count}</p>
                            </div>

                            // THE "BLAZINGLY FAST" PART:
                            // In a real app, use a <VirtualList> component here.
                            // For now, we simulate a list of blocks.
                            <div class="space-y-2">
                                <h3 class="font-bold text-gray-400 mb-2">"Today's Journal"</h3>
                                <ul class="list-disc pl-5 space-y-1">
                                    <Block content="Discussing Logseq architecture with user" />
                                    <Block content="Leptos signals are extremely performant" />
                                    <Block content="Tauri v2 brings this power to Android" />
                                    <Block content="TODO: Implement markdown parsing in core crate" />
                                </ul>
                            </div>
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}

// Granular Component: Updates to this component won't re-render the parent
#[component]
fn Block(#[prop(into)] content: String) -> impl IntoView {
    // You can add individual signals here for editing state
    let (editing, set_editing) = create_signal(false);

    view! {
        <li
            class="hover:text-indigo-300 cursor-pointer transition-colors"
            on:click=move |_| set_editing.update(|e| *e = !*e)
        >
            {move || if editing.get() {
                view! { <input class="bg-gray-700 text-white p-1 rounded" value=content.clone() /> }.into_view()
            } else {
                view! { <span>{content.clone()}</span> }.into_view()
            }}
        </li>
    }
}
