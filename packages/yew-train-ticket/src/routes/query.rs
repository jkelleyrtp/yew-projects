use crate::components::header::Header;
use urlparse::urlparse;
// use urlencoding::decode;
use fc_macros::functional_component;
use urlparse::GetQuery;
use yew::{html, Callback, Html, MouseEvent};
use yew_functional::{FunctionComponent, FunctionProvider};
use yew_router::service::RouteService;

#[functional_component]
pub fn query() -> Html {
    let route_service: RouteService<String> = RouteService::new();
    let query = route_service.get_query();
    let url = urlparse(&query);
    let query = url.get_parsed_query().unwrap();
    let from: String = query.get_first_from_str("from").unwrap();
    let to: String = query.get_first_from_str("to").unwrap();
    let date: String = query.get_first_from_str("date").unwrap();
    let high_speed: String = query.get_first_from_str("high_speed").unwrap();

    let window = web_sys::window().unwrap();
    let history = window
        .history()
        .expect("browser does not support history API");

    let onclick: Callback<MouseEvent> = Callback::from(move |_| {
        history.back();
        ()
    });

    return html! {
        <div class="header-wrapper">
             <Header title=format!("{} > {}",from,to) onback=Some(onclick) />
             {date}{high_speed}
        </div>

    };
}
