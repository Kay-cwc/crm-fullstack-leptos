use leptos::*;

use crate::models::customer::{ Customer, CustomerForm };
use crate::components::{
    buttons::{ Button, ButtonVariant, ButtonType },
    input::TextField,
};

#[component]
fn InlineCustomerForm<F, H>(
    // pure component that control the flow only. does not impl logic on the state
    customer: ReadSignal<CustomerForm>,
    set_customer: WriteSignal<CustomerForm>,
    on_confirm: F,
    on_cancel: H,
) -> impl IntoView 
where 
    F: Fn(ev::MouseEvent) + 'static,
    H: Fn(ev::MouseEvent) + 'static
{
    let disable_confirm = create_memo(move |_| customer.get().first_name.is_empty() || customer.get().last_name.is_empty());
    view! {
        <TextField 
            extend_clx="mx-2".to_string()
            value=Signal::derive(move || customer.get().first_name)
            placeholder="First Name".to_string()
            on_change=move |v| set_customer.update(|_v| _v.first_name = v)
        />
        <TextField 
            extend_clx="mx-2".to_string()
            value=Signal::derive(move || customer.get().last_name)
            placeholder="Last Name".to_string()
            on_change=move |v| set_customer.update(|_v| _v.last_name = v)
        />
        <Button 
            label="Save".to_string()
            on_click=on_confirm
            disabled=disable_confirm
        />
        <Button 
            variant=ButtonVariant::Outlined
            label="Cancel".to_string()
            on_click=on_cancel
        />
    }
}

const DEFAULT_CUSTOMERS: [&'static str; 5] = ["Alice", "Bob", "Carol", "Dave", "Eve"];

#[component]
pub fn HomePage() -> impl IntoView {
    // STATES

    // map the default customers array into a data class list with nested signals
    let init_customer_list = (0..DEFAULT_CUSTOMERS.len())
        .map(|id| 
            create_signal(Customer::new(
                id,
                &CustomerForm { first_name: DEFAULT_CUSTOMERS[id].to_string(), last_name: "" .to_string() },
            ))
        )
        .collect::<Vec<_>>();

    // create a signal for the customers list
    let (customer_list, set_customer_list) = create_signal(init_customer_list);

    // input for new customer 
    let (new_customer, set_new_customer) = create_signal(CustomerForm::empty());    
    let (show_form, set_show_form) = create_signal(false);

    let insert_customer = move |_| {
        if !new_customer.with(|v| v.first_name.is_empty() || v.last_name.is_empty()) {
            // validation here. clear the input field after insert
            set_customer_list.update(|c| c.push(
                create_signal(Customer::new(c.len(), &new_customer.get()))
            ));
            set_new_customer.update(|_v| *_v = CustomerForm::empty());
            set_show_form(false);
        }
    };

    let handle_cancel = move |_| {
        set_new_customer.update(|_v| *_v = CustomerForm::empty());
        set_show_form(false)
    };

    view! {
        <div class="mx-2 my-4">
            {move || match show_form.get() {
                true => view! {
                    <InlineCustomerForm 
                        customer=new_customer
                        set_customer=set_new_customer
                        on_confirm=insert_customer
                        on_cancel=handle_cancel
                    />
                }.into_view(),
                false => view! { 
                    <Button 
                        label="Add".to_string()
                        on_click=move |_| set_show_form(true)
                    />
                }
            }}
            <For 
                each=move || customer_list.get()
                key=|(customer, _)| customer.get().id // the id
                children=move |(customer, set_customer)| {
                    let (editing, set_editing) = create_signal(false);
                    // create a form to store the tmp value
                    // the value will copy from customer signal before edit,
                    // and be copied to the customer signal on confirm
                    let (form, set_form) = create_signal(CustomerForm::empty());
                    let start_edit = move |_| {
                        set_form.update(|v| {
                            v.first_name = customer.get().first_name.clone();
                            v.last_name = customer.get().last_name.clone();
                        });
                        set_editing(true);
                    };

                    view! {
                        <div class="my-2">
                            {move || match editing.get() {
                                // display edit form
                                true => view! {
                                    <InlineCustomerForm 
                                        customer=form
                                        set_customer=set_form
                                        on_confirm=move |_| {
                                            set_customer.update(|v| {
                                                v.first_name = form.get().first_name.clone();
                                                v.last_name = form.get().last_name.clone();
                                            });
                                            set_editing(false);
                                        }
                                        on_cancel=move |_| set_editing(false)
                                    />
                                }.into_view(),
                                // display customer info
                                false => view! {
                                    <span class="font-bold">{move || format!("{} {}", customer.get().first_name, customer.get().last_name)}</span>
                                    <Button
                                        label="Edit"
                                        on_click=start_edit
                                    />
                                    <Button
                                        button_type=ButtonType::Warning
                                        label="Delete"
                                        // NOTE: the remain method is like filter. 
                                        // but instead of returning the filtered list, it takes a mutable ref to the original list
                                        on_click=move |_| set_customer_list.update(|c_list| c_list.retain(|&(c, _)| c.get().id != customer.get().id ))
                                    />
                                }.into_view()
                            }}
                        </div>
                    }
                }
            />
        </div>
    }
}
