use leptos::*;

use crate::utils::common::classnames;

#[component]
fn ContainedButton<F>(
    #[prop(into)]
    label: MaybeSignal<String>,
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
            class=classnames(["contained bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mx-2 rounded transition".to_string(), extend_clx].to_vec())
            on:click=on_click
        >
            {label}
        </button>
    }
}

#[component]
fn OutlinedButton<F>(
    #[prop(into)]
    label: MaybeSignal<String>,
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
            class=classnames(["outlined bg-white hover:bg-blue-700 text-blue-500 hover:text-white font-semibold py-2 px-4 mx-2 ring ring-blue-500 hover:ring-blue-700 rounded transition".to_string(), extend_clx].to_vec())
            on:click=on_click
        >
            {label}
        </button>
    }
}

pub enum ButtonVariant {
    Contained,
    Outlined,
}

/**
 * a match-cased button component
 * @todo add more variants
 */
#[component]
pub fn Button<F>(
    // when you dont want a signal
    #[prop(into)]
    label: MaybeSignal<String>,
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
        ButtonVariant::Outlined => view! {
            <OutlinedButton
                label=label
                on_click=on_click
                extend_clx=extend_clx
            />
        },
    }
}