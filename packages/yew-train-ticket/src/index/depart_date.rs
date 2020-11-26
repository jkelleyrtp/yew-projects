use crate::store::store::{Action, StoreDispatch, StoreModel};
use chrono::prelude::*;
use fc_macros::functional_component;
use yew::{html, Callback, Html, Properties};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};

use std::rc::Rc;

#[functional_component]
fn depart_date() -> Html {
    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();
    let StoreModel {
        local_time: date_time,
        ..
    } = &***ctx;

    let week = date_time.weekday();
    let time = date_time.format("%Y-%m-%d").to_string();
    let weekday_str = match week {
        chrono::Weekday::Mon => "周一",
        chrono::Weekday::Tue => "周二",
        chrono::Weekday::Wed => "周三",
        chrono::Weekday::Thu => "周四",
        chrono::Weekday::Fri => "周五",
        chrono::Weekday::Sat => "周六",
        chrono::Weekday::Sun => "周日",
    };

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::ToggleDateSelectorVisible);
            return ();
        }
        _ => (),
    });

    let now = Local::now();
    let is_today = now.month() == date_time.month() && now.day() == date_time.day();

    return html! {
        <div class="depart-date"
        onclick=onclick
        >
            <input type="hidden" name="date"
            value=time
            />
            {time}
            <span class="depart-week">{weekday_str}{ if is_today {"(今天)"} else {""} }</span>
        </div>
    };
}
