use leptos::*;

#[component]
pub fn TextField<F>(
    #[prop(into)]
    value: ReadSignal<String>,
    on_change: F,
) -> impl IntoView
    where F: Fn(String) + 'static 
{
    view! {
        <input 
            type="text"
            on:input=move |e| on_change(event_target_value(&e))
            prop:value=value
        />
    }
}