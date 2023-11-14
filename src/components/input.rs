use leptos::*;

use crate::utils::common::classnames;

#[component]
pub fn TextField<F>(
    #[prop(into)]
    value: Signal<String>,
    #[prop(default="".to_string())]
    placeholder: String,
    on_change: F,
    #[prop(default="".to_string())]
    extend_clx: String,
) -> impl IntoView
    where F: Fn(String) + 'static 
{
    view! {
        <input 
            class=classnames(["ring-2 ring-inset ring-gray-400 rounded-md p-2".to_string(), extend_clx].to_vec())
            type="text"
            on:input=move |e| on_change(event_target_value(&e))
            prop:value=value
            prop:placeholder=placeholder
        />
    }
}