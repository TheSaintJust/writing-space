use leptos::prelude::*;
use leptos::mount::mount_to_body;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const STORAGE_KEY: &str = "writing-space.draft.v1";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct SentenceBlock {
    id: u64,
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Draft {
    title: String,
    sentences: Vec<SentenceBlock>,
}

fn starter_draft() -> Draft {
    Draft {
        title: "The case for better public institutions".to_string(),
        sentences: vec![
            SentenceBlock { id: 1, text: "Public institutions shape how citizens experience the state in everyday life.".to_string() },
            SentenceBlock { id: 2, text: "Their performance depends not only on formal rules, but also on how decisions are made and reviewed.".to_string() },
            SentenceBlock { id: 3, text: "Transparency can help identify irregularities, although information alone does not guarantee accountability.".to_string() },
            SentenceBlock { id: 4, text: "A useful reform agenda should therefore connect public information with clear responsibility and meaningful oversight.".to_string() },
            SentenceBlock { id: 5, text: String::new() },
        ],
    }
}

fn load_draft() -> Draft {
    let stored = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten());

    stored
        .and_then(|value| serde_json::from_str::<Draft>(&value).ok())
        .unwrap_or_else(starter_draft)
}

fn save_draft(draft: &Draft) -> bool {
    let Some(storage) = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten()) else {
        return false;
    };

    match serde_json::to_string(draft) {
        Ok(value) => storage.set_item(STORAGE_KEY, &value).is_ok(),
        Err(_) => false,
    }
}

fn word_count(sentences: &[SentenceBlock]) -> usize {
    sentences.iter().map(|sentence| sentence.text.split_whitespace().count()).sum()
}

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();

    let initial = load_draft();
    let (title, set_title) = signal(initial.title);
    let (sentences, set_sentences) = signal(initial.sentences);
    let (selected_id, set_selected_id) = signal(1_u64);
    let (save_message, set_save_message) = signal("Saved in this browser");
    let (show_notice, set_show_notice) = signal(true);
    let (search, set_search) = signal(String::new());

    Effect::new(move |_| {
        let draft = Draft {
            title: title.get(),
            sentences: sentences.get(),
        };
        if save_draft(&draft) {
            set_save_message.set("Saved in this browser");
        } else {
            set_save_message.set("Could not save in this browser");
        }
    });

    let word_total = move || word_count(&sentences.get());
    let sentence_total = move || sentences.get().iter().filter(|s| !s.text.trim().is_empty()).count();

    let add_sentence = move |after_id: Option<u64>| {
        let new_id = js_sys::Date::now() as u64;
        set_sentences.update(|items| {
            let block = SentenceBlock { id: new_id, text: String::new() };
            if let Some(after) = after_id {
                if let Some(index) = items.iter().position(|item| item.id == after) {
                    items.insert(index + 1, block);
                } else {
                    items.push(block);
                }
            } else {
                items.push(block);
            }
        });
        set_selected_id.set(new_id);
    };

    let move_sentence = move |id: u64, direction: isize| {
        set_sentences.update(|items| {
            if let Some(index) = items.iter().position(|item| item.id == id) {
                let next = index as isize + direction;
                if next >= 0 && (next as usize) < items.len() {
                    items.swap(index, next as usize);
                }
            }
        });
        set_selected_id.set(id);
    };

    let delete_sentence = move |id: u64| {
        set_sentences.update(|items| {
            items.retain(|item| item.id != id);
            if items.is_empty() {
                items.push(SentenceBlock { id: js_sys::Date::now() as u64, text: String::new() });
            }
        });
    };

    let export_markdown = move |_| {
        let current_title = title.get();
        let current_sentences = sentences.get();
        let body = current_sentences
            .iter()
            .map(|sentence| sentence.text.trim())
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
        let markdown = format!("# {}\n\n{}\n", current_title, body);
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let encoded = js_sys::encode_uri_component(&markdown);
                if let Ok(link) = document.create_element("a") {
                    let _ = link.set_attribute("href", &format!("data:text/markdown;charset=utf-8,{}", encoded));
                    let filename = current_title
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else if c.is_whitespace() { '-' } else { c })
                        .collect::<String>();
                    let _ = link.set_attribute("download", &format!("{}.md", filename.trim_matches('-')));
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&link);
                        if let Ok(link) = link.dyn_into::<web_sys::HtmlElement>() {
                            link.click();
                            link.remove();
                        }
                    }
                }
            }
        }
    };

    view! {
        <div class="app-shell">
            <aside class="sidebar">
                <div class="brand-lockup">
                    <div class="brand-mark" aria-hidden="true"><span></span></div>
                    <div>
                        <div class="brand-name">"writing space"</div>
                        <div class="brand-caption">"THINK ON THE PAGE"</div>
                    </div>
                </div>

                <div class="workspace-label">"WORKSPACE" <span class="tiny-dot"></span></div>
                <button class="new-document-button" on:click=move |_| {
                    set_title.set("Untitled document".to_string());
                    set_sentences.set(vec![SentenceBlock { id: js_sys::Date::now() as u64, text: String::new() }]);
                    set_selected_id.set(js_sys::Date::now() as u64);
                }>
                    <span class="plus-symbol">"＋"</span><span>"New draft"</span><span class="shortcut">"LOCAL"</span>
                </button>

                <div class="sidebar-section-title"><span>"YOUR DRAFT"</span><span class="count-pill">"1"</span></div>
                <div class="search-field">
                    <span class="search-icon" aria-hidden="true">"⌕"</span>
                    <input
                        prop:value=move || search.get()
                        on:input=move |ev| set_search.set(event_target_value(&ev))
                        placeholder="Find text in this draft…"
                        aria-label="Find text in this draft"
                    />
                </div>
                <div class="document-list">
                    <div class="document-nav-item active">
                        <span class="doc-icon">"¶"</span>
                        <span class="document-nav-copy">
                            <span class="document-nav-title">{move || title.get()}</span>
                            <span class="document-nav-meta">{move || format!("{} sentence blocks", sentences.get().len())}</span>
                        </span>
                        <span class="active-indicator"></span>
                    </div>
                    <p class="sidebar-hint">"Multiple documents and team sharing will be added in a later step."</p>
                </div>

                <div class="sidebar-bottom">
                    <div class="team-card">
                        <div class="team-avatars"><span>"Y"</span><span>"+"</span></div>
                        <div class="team-card-copy"><strong>"Your writing team"</strong><span>"Collaboration comes later"</span></div>
                        <span class="team-card-arrow">"↗"</span>
                    </div>
                    <div class="profile-row">
                        <div class="profile-avatar">"W"</div>
                        <div class="profile-copy"><strong>"Writer"</strong><span>"Local prototype"</span></div>
                    </div>
                </div>
            </aside>

            <main class="main-area">
                <header class="topbar">
                    <div class="breadcrumbs"><span>"Workspace"</span><span class="crumb-slash">"/"</span><strong>"Draft"</strong></div>
                    <div class="topbar-actions">
                        <div class="save-status"><span class="save-status-dot"></span>{move || save_message.get()}</div>
                        <button class="export-button" on:click=export_markdown><span aria-hidden="true">"↧"</span>" Export"</button>
                    </div>
                </header>

                <Show when=move || show_notice.get()>
                    <div class="privacy-banner">
                        <span class="privacy-icon" aria-hidden="true">"ⓘ"</span>
                        <span><strong>"Early prototype. "</strong>"Your draft is saved in this browser only. It is not yet shared with a team or stored in the cloud."</span>
                        <button on:click=move |_| set_show_notice.set(false) aria-label="Dismiss notice">"×"</button>
                    </div>
                </Show>

                <div class="editor-layout">
                    <section class="editor-column">
                        <div class="document-kicker"><span class="kicker-line"></span>" WORKING DOCUMENT " <span class="kicker-dot">"·"</span> <span>"EDITING"</span></div>
                        <input
                            class="document-title-input"
                            prop:value=move || title.get()
                            on:input=move |ev| set_title.set(event_target_value(&ev))
                            aria-label="Document title"
                            placeholder="Untitled document"
                        />
                        <div class="document-metadata">
                            <span class="status-chip"><span></span>" Draft"</span>
                            <span class="metadata-divider"></span>
                            <span>{move || format!("{} words", word_total())}</span>
                            <span class="metadata-divider"></span>
                            <span>{move || format!("{} sentence blocks", sentence_total())}</span>
                        </div>

                        <div class="writing-rule"><span>"YOUR ARGUMENT, ONE SENTENCE AT A TIME"</span><span class="rule-line"></span></div>

                        <div class="sentence-list">
                            <For
                                each=move || sentences.get()
                                key=|sentence| sentence.id
                                children=move |sentence| {
                                    let id = sentence.id;
                                    let number = move || sentences.get().iter().position(|item| item.id == id).map(|i| i + 1).unwrap_or(1);
                                    let selected = move || selected_id.get() == id;
                                    view! {
                                        <div class=move || if selected() { "sentence-row selected" } else { "sentence-row" }>
                                            <div class="sentence-number">{move || format!("{:02}", number())}</div>
                                            <div class="sentence-content">
                                                <textarea
                                                    prop:value=sentence.text.clone()
                                                    on:focus=move |_| set_selected_id.set(id)
                                                    on:input=move |ev| {
                                                        let value = event_target_value(&ev);
                                                        set_sentences.update(|items| {
                                                            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                                                item.text = value;
                                                            }
                                                        });
                                                        set_selected_id.set(id);
                                                    }
                                                    placeholder="Write one sentence or thought…"
                                                    rows=if sentence.text.len() > 95 { 3 } else if sentence.text.len() > 48 { 2 } else { 1 }
                                                    aria-label="Sentence block"
                                                ></textarea>
                                                <div class="sentence-tools">
                                                    <button on:click=move |_| move_sentence(id, -1) title="Move sentence up" aria-label="Move sentence up">"↑"</button>
                                                    <button on:click=move |_| move_sentence(id, 1) title="Move sentence down" aria-label="Move sentence down">"↓"</button>
                                                    <span class="tool-separator"></span>
                                                    <button on:click=move |_| add_sentence(Some(id)) title="Insert a sentence below" aria-label="Insert a sentence below">"＋"</button>
                                                    <button class="delete-tool" on:click=move |_| delete_sentence(id) title="Delete sentence" aria-label="Delete sentence">"×"</button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>

                        <button class="add-sentence-button" on:click=move |_| add_sentence(Some(selected_id.get()))>
                            <span class="add-sentence-plus">"＋"</span><span>"Add a sentence"</span><span class="add-sentence-hint">"Keep building your thought"</span>
                        </button>

                        <div class="editor-footer">
                            <span><span class="footer-check">"✓"</span>" Your structure stays in your control."</span>
                            <span>"WRITING SPACE " <span class="footer-separator">"/"</span> " 0.1"</span>
                        </div>
                    </section>

                    <aside class="companion-column">
                        <div class="companion-heading"><div><span class="companion-sparkle">"✳"</span><span>"Writing companion"</span></div><span class="preview-badge">"PREVIEW"</span></div>
                        <p class="companion-intro">"A little space to think about the shape of your argument."</p>

                        <div class="companion-card focus-card">
                            <div class="card-eyebrow"><span class="card-icon">"◎"</span>" DOCUMENT OUTLINE"</div>
                            <div class="outline-stats"><strong>{move || sentence_total()}</strong><span>"sentence blocks"</span></div>
                            <div class="outline-preview">
                                <For
                                    each=move || sentences.get().into_iter().filter(|sentence| !sentence.text.trim().is_empty()).collect::<Vec<_>>()
                                    key=|sentence| sentence.id
                                    children=move |sentence| {
                                        let id = sentence.id;
                                        view! {
                                            <button class="outline-item" on:click=move |_| set_selected_id.set(id)>
                                                <span class="outline-number">{move || format!("{:02}", sentences.get().iter().position(|item| item.id == id).map(|i| i + 1).unwrap_or(1))}</span>
                                                <span>{sentence.text}</span>
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>

                        <div class="companion-card next-card">
                            <div class="card-eyebrow"><span class="card-icon">"⌁"</span>" NEXT TO YOUR DRAFT"</div>
                            <div class="planned-feature"><span class="feature-symbol source-symbol">"↗"</span><div><strong>"Sources & citations"</strong><p>"Keep evidence beside the sentence it supports."</p></div><span class="feature-lock">"SOON"</span></div>
                            <div class="planned-feature"><span class="feature-symbol review-symbol">"✳"</span><div><strong>"Argument review"</strong><p>"Find gaps, assumptions and links between ideas."</p></div><span class="feature-lock">"SOON"</span></div>
                        </div>

                        <div class="gentle-prompt">
                            <div class="prompt-icon">"✦"</div>
                            <div><strong>"A thought to keep nearby"</strong><p>"What is the point you want your reader to understand—and what would convince them?"</p></div>
                        </div>
                        <div class="companion-note"><span class="note-dot"></span>" This is your space. Suggestions will support your thinking, never replace it."</div>
                    </aside>
                </div>
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
