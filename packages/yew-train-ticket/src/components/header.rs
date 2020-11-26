// use crate::routes::AppRoute;
use yew::{html, Callback, Html, MouseEvent, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub onback: Option<Callback<MouseEvent>>,
}

pub struct HeaderFC {}
pub type Header = FunctionComponent<HeaderFC>;
impl FunctionProvider for HeaderFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let Props { title, onback } = &props;

        let onclick: Callback<MouseEvent> = match onback {
            Some(onclick) => onclick.clone(),
            _ => Callback::from(|_| ()),
        };

        return html! {
            <div class="header">
                <div class="header-back"  style=" width=42 height=42" onclick=onclick>
                {"<"}
                </div>
                <h1 class="header-title">{title}</h1>
            </div>
        };
    }
}
