use leptos::prelude::*;
use tag_app::{TagCatalogPicker, TAGCATALOGPICKER_DOC, TAGCATALOGPICKER_PROPS};
use uf_product::components::{Caption1, ComponentPreviewCard, OrbitalComponentView};

/// Preview page for `tag_app`'s `TagCatalogPicker` (behind the `tag-catalog` feature).
#[component]
pub fn TagCatalogPickerPreview() -> impl IntoView {
    let selected = RwSignal::new(Vec::<String>::new());
    let on_change = Callback::new(|_ids: Vec<String>| {
        // Product apps wire connection APIs here; selection lives in `selected`.
    });

    view! {
        <OrbitalComponentView
            component_name="Tag Catalog Picker"
            component_description=TAGCATALOGPICKER_DOC
            component_props=TAGCATALOGPICKER_PROPS
            default_code=r#"
use tag_app::TagCatalogPicker;

let selected = RwSignal::new(Vec::<String>::new());
view! {
    <TagCatalogPicker
        selected=selected
        on_change=Callback::new(|ids| { /* connection APIs */ })
    />
}
"#
            default=view! {
                <div data-testid="tag-catalog-picker-preview">
                    <TagCatalogPicker selected=selected on_change=on_change />
                </div>
            }
        >
            <ComponentPreviewCard title="Selection state">
                <Caption1>
                    "Selected: "
                    {move || format!("{:?}", selected.get())}
                </Caption1>
            </ComponentPreviewCard>
        </OrbitalComponentView>
    }
}
