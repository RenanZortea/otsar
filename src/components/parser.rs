// src/components/parser.rs
use leptos::*;
use regex::Regex;

/// Represents a segment of text: either plain text or a styled block.
#[derive(Clone, Debug, PartialEq)]
pub enum TextSegment {
    Plain(String),
    Styled { content: String, classes: String },
}

/// Parses the input string for the pattern $(content, classes)
pub fn parse_custom_syntax(input: &str) -> Vec<TextSegment> {
    // Regex to match $(content, classes)
    // Captures: 1 = content, 2 = classes
    let re = Regex::new(r"\$\(([^,]+),\s*([^)]+)\)").unwrap();

    let mut segments = Vec::new();
    let mut last_end = 0;

    for cap in re.captures_iter(input) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();

        // Push preceding plain text if any
        if match_start > last_end {
            segments.push(TextSegment::Plain(input[last_end..match_start].to_string()));
        }

        // Push the styled segment
        segments.push(TextSegment::Styled {
            content: cap[1].trim().to_string(),
            classes: cap[2].trim().to_string(),
        });

        last_end = match_end;
    }

    // Push remaining text
    if last_end < input.len() {
        segments.push(TextSegment::Plain(input[last_end..].to_string()));
    }

    segments
}

/// Component to render the parsed text
#[component]
pub fn ParsedText(text: ReadSignal<String>) -> impl IntoView {
    // Memoize the parsing so it only runs when text changes
    let segments = create_memo(move |_| parse_custom_syntax(&text.get()));

    view! {
        <div class="whitespace-pre-wrap">
            {move || {
                segments.get().into_iter().map(|segment| {
                    match segment {
                        TextSegment::Plain(content) => view! { <span>{content}</span> }.into_view(),
                        TextSegment::Styled { content, classes } => {
                            view! { <span class=classes>{content}</span> }.into_view()
                        }
                    }
                }).collect_view()
            }}
        </div>
    }
}
