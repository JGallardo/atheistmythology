use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};
use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, };
#[component] 
pub fn NavBar() -> impl IntoView {
    view! {
        <nav>"this is a navbar"</nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/atheistmythology.css"/>
        <Title text="Welcome to Atheist Mythology"/>

        <NavBar/>
        
        <Router>
            <main>
                <Routes>
                    <Route path="" view=BlogPreviews/>
                    <Route path="/edit/:post_id?" view=EditPost/>
                    // <Route path="/view/:post_id?" view=ViewPost/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
