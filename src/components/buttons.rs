use leptos::*;

#[component]
fn ContainedButton<F>(
    #[prop(into)]
    label: Signal<String>,
    // callback fn with MouseEvent as argument
    on_click: F,
    #[prop(default="".to_string())]
    extend_clx: String,
) -> impl IntoView 
where
    F: Fn(ev::MouseEvent) + 'static
{
    view! {
        <button 
            class=format!("contained bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mx-2 rounded {}", extend_clx)
            on:click=on_click
        >
            {label}
        </button>
    }
}

pub enum ButtonVariant {
    Contained,
}

/**
 * a match-cased button component
 * @todo add more variants
 */
#[component]
pub fn Button<F>(
    #[prop(into)]
    label: Signal<String>,
    on_click: F,
    #[prop(default=ButtonVariant::Contained)]
    variant: ButtonVariant,
    // allow override the css style of the base class
    #[prop(default="".to_string())]
    extend_clx: String,
) -> impl IntoView 
where 
    F: Fn(ev::MouseEvent) + 'static 
{
    match variant {
        ButtonVariant::Contained => view! {
            <ContainedButton
                label=label
                on_click=on_click
                extend_clx=extend_clx
            />
        },   
    }
}