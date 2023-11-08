use leptos::*;

#[component]
fn ContainedButton<F>(
    #[prop(into)]
    label: Signal<String>,
    on_click: F,
) -> impl IntoView 
where
    F: Fn(ev::MouseEvent) + 'static
{
    view! {
        <button 
            class="contained" 
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
) -> impl IntoView 
where 
    F: Fn(ev::MouseEvent) + 'static 
{
    match variant {
        ButtonVariant::Contained => view! {
            <ContainedButton
                label=label
                on_click=on_click
            />
        },   
    }
}