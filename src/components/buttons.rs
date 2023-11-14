use leptos::*;

use crate::utils::common::classnames;

const BASE_BTN_CLX: &str = "font-bold py-2 px-4 mx-2 rounded transition";

struct BtnStyle {
    button_type: ButtonType,
    variant: ButtonVariant,
    min_width: bool,
}

impl BtnStyle {
    fn get_clx(&self, extra_clx: String) -> Vec<String> {
        // 1. base btn style: padding, margin, shape, font-weight and transition
        let mut class_list = vec![BASE_BTN_CLX.to_string()];
        // 2. variant style: bg color, text color, border color
        let mut variant_style = match self.variant {
            ButtonVariant::Contained => {
                let btn_type_style = match self.button_type {
                    ButtonType::Standard => "bg-blue-500 hover:bg-blue-700 ".to_string(),
                    ButtonType::Warning => "bg-red-500 hover:bg-red-700 ".to_string(),
                };
                let text_color = "text-white".to_string();
                
                vec![btn_type_style, text_color]
            },
            ButtonVariant::Outlined => vec!["text-blue-500 hover:text-white outlined bg-white hover:bg-blue-700 ring ring-inset ring-blue-500 hover:ring-blue-700".to_string()],
        };

        match self.min_width {
            true => variant_style.push("w-20 min-w-fit".to_string()),
            false => (),
        }

        class_list.append(&mut variant_style);
        class_list.push(extra_clx);

        class_list
    }
}

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
    let class_list = BtnStyle {
        button_type,
        variant: ButtonVariant::Contained,
        min_width: true,
    }.get_clx(extend_clx);

    view! {
        <button 
            class=classnames(class_list)
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
    button_type: ButtonType,
) -> impl IntoView 
where
    F: Fn(ev::MouseEvent) + 'static
{
    let class_list: Vec<String> = BtnStyle {
        button_type,
        variant: ButtonVariant::Outlined,
        min_width: true,
    }.get_clx(extend_clx);

    view! {
        <button 
            class=classnames(class_list)
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
                button_type=button_type
                extend_clx=extend_clx
            />
        },
    }
}