use crate::{
    components::header::Header,
    store::store::{Action, StoreDispatch, StoreModel},
};
use chrono::prelude::*;
use fc_macros::functional_component;
use std::rc::Rc;
use yew::{html, Callback, Html, MouseEvent, Properties};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};

#[derive(Properties, Clone, PartialEq)]
pub struct DayProps {
    pub date: Option<DateTime<Local>>,
}

#[functional_component]
fn day(props: &DayProps) -> Html {
    let DayProps { date } = &props;
    let now = Local::now();

    let (day, day_str, is_today, prev_class, weekend_class) = match *date {
        Some(day) => {
            let weekend_class = match day.weekday() {
                chrono::Weekday::Sat | chrono::Weekday::Sun => "weekend",
                _ => "",
            };

            (
                day,
                day.day().to_string(),
                now.month() == day.month() && now.day() == day.day(),
                if day.day() < now.day() && now.month() == day.month() {
                    "disabled"
                } else {
                    ""
                },
                weekend_class,
            )
        }
        None => (Local::now(), "".to_string(), false, "", ""),
    };

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            if day.day() >= now.day() || day.month() > now.month() {
                let dispatch = &*dispatch;
                dispatch.emit(Action::SelectDate(day));
                dispatch.emit(Action::ToggleDateSelectorVisible);
            }
            return ();
        }
        _ => (),
    });

    return html! {
        <td
        onclick=onclick
        class={format!("{} {}",weekend_class,prev_class)}
        >
        { if is_today { "今天".to_string() } else { day_str } }
     </td>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct WeekProps {
    pub week: Vec<Option<DateTime<Local>>>,
}

#[functional_component]
fn week(props: &WeekProps) -> Html {
    let WeekProps { week } = &props;

    return html! {
        <tr
        class="date-table-days"
        >
        {for week.iter().map(|date| {
            html! { <Day date=date /> }
        })}
    </tr>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct MonthProps {
    pub date: DateTime<Local>,
}

#[functional_component]
fn month(props: &MonthProps) -> Html {
    let MonthProps { date } = &props;
    let month = date.month();
    let next_month_date = date.with_month(month + 1).unwrap_or(*date);
    let year_month = date.format("%Y年%m月").to_string();
    let first_day_in_month = date.with_day(1).unwrap_or(*date);
    let mut day_in_month = date.with_day(1).unwrap_or(*date);
    let mut day_list: Vec<Option<DateTime<Local>>> = vec![];

    let weekday = first_day_in_month.weekday();
    let offset = match weekday {
        chrono::Weekday::Mon => 0,
        chrono::Weekday::Tue => 1,
        chrono::Weekday::Wed => 2,
        chrono::Weekday::Thu => 3,
        chrono::Weekday::Fri => 4,
        chrono::Weekday::Sat => 5,
        chrono::Weekday::Sun => 6,
    };

    for _ in 0..offset {
        day_list.push(None);
    }

    // 先算出当月日期数组
    while month == day_in_month.month() {
        let day = day_in_month.day();
        day_list.push(Some(day_in_month));
        day_in_month = day_in_month.with_day(day + 1).unwrap_or(next_month_date);
    }

    let rest = day_list.iter().len() % 7;

    for _ in 0..(7 - rest) {
        day_list.push(None);
    }

    let len = day_list.iter().len();

    let rows = len / 7;

    let matrix: Vec<Vec<Option<DateTime<Local>>>> = (0..rows)
        .map(|i| {
            return (0..7)
                .map(|ii| {
                    let xy = i * 7 + ii;
                    return day_list[xy];
                })
                .collect();
        })
        .collect();

    return html! {
        <table class="date-table">
        <thead>
            <tr>
                <td colSpan="7">
                    <h5>
                     {year_month}
                    </h5>
                </td>
            </tr>
        </thead>
        <tbody>
            <tr class="data-table-weeks">
                <th>{"周一"}</th>
                <th>{"周二"}</th>
                <th>{"周三"}</th>
                <th>{"周四"}</th>
                <th>{"周五"}</th>
                <th class="weekend">{"周六"}</th>
                <th class="weekend">{"周日"}</th>
            </tr>
            {for matrix.iter().map(|week| {
                html! { <Week week=week /> }
            })}
        </tbody>
    </table>
    };
}

#[functional_component]
fn date_selector() -> Html {
    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();
    let StoreModel {
        date_selector_visible: show,
        ..
    } = &***ctx;

    let local_time = Local::now();
    let month = local_time.month();
    let local_time1 = local_time.with_month(month + 1).unwrap_or(local_time);
    let local_time2 = local_time.with_month(month + 2).unwrap_or(local_time);

    let date_list: Vec<DateTime<Local>> = vec![local_time, local_time1, local_time2];

    let hidden_class = if *show { "" } else { "hidden" };

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::ToggleDateSelectorVisible);
            return ();
        }
        _ => (),
    });

    return html! {
        <div class=format!("date-selector {}", hidden_class) >
        <Header title="日期选择"
          onback=Some(onclick)
        />
        <div class="date-selector-tables">
        {for date_list.iter().map(|date| {
            html! { <Month date=date /> }
        })}
        </div>
    </div>
    };
}
