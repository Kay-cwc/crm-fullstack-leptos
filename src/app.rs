use leptos::*;
use leptos_router::*;
use leptos_meta::*;

use crate::pages::{
    homepage::HomePage,
    formpage::FormPage,
};

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