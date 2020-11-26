use crate::{
    components::header::Header,
    index::{
        city_selector::CitySelector, date_selector::DateSelector, depart_date::DepartDate,
        high_speed::HighSpeed, journey::Journey, submit::Submit,
    },
};

use crate::store::store::{reducer, StoreDispatch, StoreModel};
use chrono::prelude::*;

use std::rc::Rc;

use fc_macros::functional_component;
use yew::{html, Callback, Html, MouseEvent};
use yew_functional::{use_reducer_with_init, ContextProvider, FunctionComponent, FunctionProvider};

#[functional_component]
fn index() -> Html {
    let initail_state = StoreModel {
        from: "北京".to_string(),
        to: "上海".to_string(),
        local_time: Local::now(),
        is_high_speed: true,
        date_selector_visible: false,
        city_selector_visible: false,
        is_selecting_from: false,
    };

    let (store, dispatch) =
        use_reducer_with_init(reducer, initail_state, |initail_state: StoreModel| {
            initail_state
        });

    let dispatch = StoreDispatch(dispatch);
    type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;
    type StoreDispatchContextProvider = ContextProvider<StoreDispatch>;

    let window = web_sys::window().unwrap();
    let history = window
        .history()
        .expect("browser does not support history API");

    let onclick = Callback::from(move |_| {
        history.back();
    });

    return html! {
        <>
            <StoreDispatchContextProvider context=dispatch>
                <StoreModelContextProvider context=store>
                    <div class="header-wrapper">
                        <Header title="火车票" onback=Some(onclick) />
                    </div>
                    <form action="./query.html" class="form">
                        <Journey/>
                        <DepartDate/>
                        <HighSpeed/>
                        <Submit />
                    </form>
                    <CitySelector
                    />
                    <DateSelector
                    />

                </StoreModelContextProvider >
            </StoreDispatchContextProvider >
        </>
    };
}
