use fc_macros::functional_component;
use yew::{html, Html};
use yew_functional::{FunctionComponent, FunctionProvider};

#[functional_component]
fn submit() -> Html {
    return html! {
        <div class="submit">
        <button type="submit" class="submit-button">
            {"搜索"}
        </button>
    </div>
    };
}
