use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::model::blog_post::Post;
use crate::component::blog_post::BlogPost;

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

fn format_dt(datetime: NaiveDateTime) -> String {
    datetime.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    let post_resource = create_resource(
        move || params.get(),
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                // if no id is in the URL path parameter, assume we are making a new post
                _ => Ok(Post::new_empty()),
            }
        },
    );

    let upsert_post = create_server_action::<UpsertPost>();
    
    view! {
        <Suspense fallback=move || view! {<p>"Loading..."</p>}>
            <ErrorBoundary fallback= move |_| view ! {<p>"Error!"</p>}
                <div class="flex h-screen">
                    <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                        <ActionForm action=upsert_post>
                            <label class="block mb-4">
                                <span>Title</span>
                                    <input
                                    class="mt-1 p-2 w-full"
                                    type="text"
                                    id="title"
                                    name="title"
                                    on:input=move |ev| {
                                        set_post.update(|post| post.title = event_target_value(&ev))
                                    }
                                    prop:value={move || post.get().title}
                                />
                            </label>
                            <label class="block mb-4">
                                <span>Entry</span>
                                <textarea class="mt-1 p-2 w-full" id="text" name="text"
                                    on:input=move |ev| {
                                        post_resource.update(|curr| {
                                            if let Some(Ok(post)) = curr {
                                                post.text = event_target_value(&ev);
                                            }
                                        });
                                    }
                                    prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.text).ok())}
                                />
                            </label>
                            <div class="flex justify-center pb-4">
                                <input
                                    type="submit"
                                    value="Submit"
                                    class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                                />
                            </div>
                        </ActionForm>
                    </div>
                    <div>
                        {move || post_resource.and_then(|post| view! {<BlogPost post=post.clone()/>})}
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
