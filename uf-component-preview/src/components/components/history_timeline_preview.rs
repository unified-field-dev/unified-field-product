use leptos::prelude::*;
use record_history_leptos::{
    e2e_record_history_empty_source, e2e_record_history_source, HistoryTimeline,
    E2E_RECORD_HISTORY_KIND, HISTORYTIMELINE_DOC, HISTORYTIMELINE_PROPS,
};
use uf_product::components::{Body1, ComponentPreviewCard, OrbitalComponentView, Subtitle2};
const DEFAULT_CODE: &str = r##"
use record_history_leptos::HistoryTimeline;
use valence::RecordId;

view! {
    <HistoryTimeline source=RecordId::new("tag", tag.id.clone()) />
}

// Narrow to one history table when you know which applies:
view! {
    <HistoryTimeline
        source=RecordId::new("tag", tag.id.clone())
        kind_filter=vec!["tag_history".into()]
        max_height="300px"
    />
}
"##;

/// History timeline preview — live `RecordHistory` fixture + empty / scroll variants.
#[component]
pub fn HistoryTimelinePreview() -> impl IntoView {
    let timeline_kinds = StoredValue::new(vec![E2E_RECORD_HISTORY_KIND.to_string()]);
    let live_source = e2e_record_history_source();
    let empty_source = e2e_record_history_empty_source();

    view! {
        <div data-testid="record-history-preview-page">
            <OrbitalComponentView
                component_name="History Timeline"
                component_description=HISTORYTIMELINE_DOC
                component_props=HISTORYTIMELINE_PROPS
                default_code=DEFAULT_CODE
                default=view! {
                    <div data-testid="record-history-preview-live">
                        <Body1>"Seed via Playwright: record_history_timeline_fixture"</Body1>
                        <HistoryTimeline
                            source=live_source.clone()
                            kind_filter=timeline_kinds.get_value()
                            max_height="300px"
                        />
                    </div>
                }
            >
                <ComponentPreviewCard
                    title="Empty state"
                    code=r##"
<HistoryTimeline
    source=RecordId::new("e2e_history_source_b", "e2e-history-empty")
    kind_filter=vec!["e2e_record_history_fixture".into()]
/>
"##
                >
                    <div data-testid="record-history-preview-empty">
                        <HistoryTimeline
                            source=empty_source.clone()
                            kind_filter=timeline_kinds.get_value()
                            max_height="300px"
                        />
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Compact scroll (180px)"
                    code=r##"
<HistoryTimeline
    source=RecordId::new("e2e_history_source_a", "e2e-history-source-001")
    kind_filter=vec!["e2e_record_history_fixture".into()]
    max_height="180px"
/>
"##
                >
                    <div data-testid="record-history-preview-scroll">
                        <Subtitle2>"Scroll to load page 2"</Subtitle2>
                        <HistoryTimeline
                            source=live_source.clone()
                            kind_filter=timeline_kinds.get_value()
                            max_height="180px"
                        />
                    </div>
                </ComponentPreviewCard>
            </OrbitalComponentView>
        </div>
    }
}
