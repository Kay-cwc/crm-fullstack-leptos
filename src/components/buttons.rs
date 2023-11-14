use leptos::*;

use crate::utils::common::classnames;

#[component]
fn ContainedButton<F>(
    #[prop(into)]
    label: MaybeSignal<String>,
    button_type: ButtonType,
    // callback fn with MouseEvent as argument
    on_click: F,
    #[prop(default="".to_string())]
    extend_clx: String,
) -> impl IntoView 
where
    F: Fn(ev::MouseEvent) + 'static
{
    let button_base_clx = "text-white font-bold py-2 px-4 mx-2 rounded transition".to_string();
    let button_type_clx = match button_type {
        ButtonType::Standard => "bg-blue-500 hover:bg-blue-700 ".to_string(),
        ButtonType::Warning => "bg-red-500 hover:bg-red-700 ".to_string(),
    }.to_string();

    view! {
        <button 
            class=classnames([button_base_clx, button_type_clx, extend_clx].to_vec())
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

pub enum ButtonType {
    Standard,
    Warning,
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
    #[prop(default=ButtonType::Standard)]
    button_type: ButtonType,
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
                button_type=button_type
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