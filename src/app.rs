use leptos::*;
use leptos_router::*;
use leptos_meta::*;

use crate::components::{
    buttons::{Button, ButtonVariant},
    input::TextField,
};

#[derive(Debug, Clone)]
struct Customer {
    id: usize,
    name: String,
    count: (ReadSignal<i32>, WriteSignal<i32>)
}

impl Customer {
    fn new(id: usize, name: String) -> Customer {
        Customer {
            id,
            name,
            count: create_signal(0),
        }
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // map the default customers array into a data class list with nested signals
    let default_customers = vec!["Alice", "Bob", "Carol", "Dave", "Eve"];
    let init_customer_list = (0..default_customers.len())
        .map(|id| Customer::new(id, default_customers[id].to_string()))
        .collect::<Vec<_>>();
    // create a signal for the customers list
    let (customer_list, set_customer_list) = create_signal(init_customer_list);

    // input for new customer 
    let (new_customer, set_new_customer) = create_signal("".to_string());

    /* @todo should insert with name later */
    let insert_customer = move |_| {
        if !new_customer.get().is_empty() {
            // validation here. clear the input field after insert
            set_customer_list.update(|c| c.push(Customer::new(c.len(), new_customer.get())));
            set_new_customer.update(|_v| *_v = "".to_string());
        }
    };
    
    view! {
        <div class="mx-2">
            <Button
                variant=ButtonVariant::Contained // just an example only. default is contained
                label=Signal::derive(move || "Insert New Customer".to_string())
                on_click=insert_customer
            />
            <TextField 
                value=new_customer
                on_change=move |v| set_new_customer.update(|_v| *_v = v)
            />
            <For 
                each=move || customer_list.get()
                key=|cusotmer| cusotmer.id // the id
                children=move |customer| {
                    let (count, set_count) = customer.count;
                    view! {
                        <div>
                            <Button
                                label=Signal::derive(move || format!("{}: {}", customer.name, count.get()))
                                on_click=move |_| set_count.update(|n| *n += 1)
                            />
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
fn FormPage() -> impl IntoView {
    view! {
        <div class="mx-2">
            <h1>"Form Page"</h1>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/create" view=FormPage/>
                </Routes>
            </main>
        </Router>
    }
}