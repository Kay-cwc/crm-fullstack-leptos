use leptos::*;

#[component]
pub fn OutterBox(
    children: Children,
) -> impl IntoView {
    view! {
        <div class="border-2 border-gray-400 rounded-md p-2">
            {children()}
        </div>
    }
}