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
            class="border-2 border-gray-400 rounded-md p-2"
            type="text"
            on:input=move |e| on_change(event_target_value(&e))
            prop:value=value
        />
    }
}