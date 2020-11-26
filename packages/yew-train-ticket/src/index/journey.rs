use crate::store::store::{Action, StoreDispatch, StoreModel};
use fc_macros::functional_component;
use std::rc::Rc;
use yew::{html, Callback, Html, MouseEvent};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};

#[functional_component]
fn journey() -> Html {
    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();
    let StoreModel { to, from, .. } = &***ctx;

    let context_dispatch = use_context::<StoreDispatch>();
    let onexchange: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::ExchangeFromTo);
            ()
        }
        _ => (),
    });

    let context_dispatch = use_context::<StoreDispatch>();
    let context_dispatch1 = context_dispatch.clone();

    let onclickfrom: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::SetIsSelectingFrom(true));
            dispatch.emit(Action::ToggleCitySelectorVisible);
            return ();
        }
        _ => (),
    });

    let onclickto: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch1 {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::SetIsSelectingFrom(false));
            dispatch.emit(Action::ToggleCitySelectorVisible);
            return ();
        }
        _ => (),
    });

    return html! {
        <div class="journey">
            <div
                class="journey-station"
                onclick=onclickfrom
            >
                <input
                    type="text"
                    readOnly =true
                    name="from"
                    value={from}
                    class="journey-input journey-from"
                />
            </div>
            <div class="journey-switch"
             onclick=onexchange
            >
                {"<>"}
            </div>
            <div
                class="journey-station"
                onclick=onclickto
            >
                <input
                    type="text"
                    readOnly=true
                    name="to"
                    value={to}
                    class="journey-input journey-to"
                />
            </div>
        </div>
    };
}
