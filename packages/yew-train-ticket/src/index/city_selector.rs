use crate::{
    service::search_city_list::{search_city_list, SearchCityResult},
    store::store::{Action, StoreDispatch, StoreModel},
};
use fc_macros::functional_component;
use std::rc::Rc;
use web_sys::console;
use yew::{html, Callback, Html, MouseEvent, Properties};
use yew_functional::{
    use_context, use_effect_with_deps, use_state, FunctionComponent, FunctionProvider,
};

use crate::service::{
    city_list::{get_city_list, City, CityResult},
    future::handle_future,
};

#[derive(Properties, Clone, PartialEq)]
pub struct SuggestItemProps {
    pub name: String,
}

#[functional_component]
pub fn suggest_item(props: &SuggestItemProps) {
    let SuggestItemProps { name } = &props;

    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();
    let StoreModel {
        is_selecting_from, ..
    } = &***ctx;

    let is_selecting_from = is_selecting_from.clone();

    let name1 = name.clone();

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick: Callback<MouseEvent> =
        Callback::from(move |_: MouseEvent| match &context_dispatch {
            Some(dispatch) => {
                let name1 = name1.clone();
                let dispatch = &*dispatch;

                if is_selecting_from {
                    dispatch.emit(Action::SelectFrom(name1));
                } else {
                    dispatch.emit(Action::SelectTo(name1));
                }
                dispatch.emit(Action::ToggleCitySelectorVisible);

                return ();
            }
            _ => (),
        });

    return html! {
        <li class="city-li"
        onclick=onclick
        >
            {name}
        </li>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct SuggestProps {
    pub search_word: String,
}

#[functional_component]
pub fn suggest(props: &SuggestProps) -> Html {
    let SuggestProps { search_word } = &props;
    let search_word = search_word.clone();
    let search_word1 = search_word.clone();

    let (search_city_data, set_search_city_data) = use_state(|| SearchCityResult::new());
    let search_result = &(*search_city_data).result;
    use_effect_with_deps(
        move |_| {
            let future = async { search_city_list(search_word1).await };

            handle_future(future, move |value| set_search_city_data(value));
            return || ();
        },
        (search_word),
    );

    html! {
        <div class="city-suggest">
            <ul class="city-suggest-ul">
                {for search_result.iter().map(|item| {
                    html! {
                        <SuggestItem
                            name={item.display.clone()}
                        />
                    }
                })}
            </ul>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AlphaIndexProps {
    pub alpha: char,
}

#[functional_component]
pub fn alpha_index(props: &AlphaIndexProps) -> Html {
    let AlphaIndexProps { alpha } = &props;
    let alpha = alpha.clone();

    let onclick: Callback<MouseEvent> = Callback::from(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let ele = document
            .query_selector(&format!("[data-cate='{}']", alpha.to_string()))
            .expect("document should have a body");

        match ele {
            Some(ele) => {
                console::log_1(&alpha.to_string().into());
                ele.scroll_into_view()
            }
            _ => (),
        };
    });

    return html! {
        <i class="city-index-item"
        onclick=onclick
        >
            {alpha}
        </i>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct CityItemProps {
    pub name: String,
}

#[functional_component]
pub fn city_item(props: &CityItemProps) -> Html {
    let CityItemProps { name } = &props;

    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();
    let StoreModel {
        is_selecting_from, ..
    } = &***ctx;

    let is_selecting_from = is_selecting_from.clone();

    let name1 = name.clone();

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick: Callback<MouseEvent> =
        Callback::from(move |_: MouseEvent| match &context_dispatch {
            Some(dispatch) => {
                let name1 = name1.clone();
                let dispatch = &*dispatch;
                if is_selecting_from {
                    dispatch.emit(Action::SelectFrom(name1));
                } else {
                    dispatch.emit(Action::SelectTo(name1));
                }
                dispatch.emit(Action::ToggleCitySelectorVisible);

                return ();
            }
            _ => (),
        });

    return html! {
        <li class="city-li"
        onclick=onclick
        >
        {name}
    </li>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct CitySectionProps {
    pub city_names: Vec<String>,
    pub title: String,
}

#[functional_component]
pub fn city_section(props: &CitySectionProps) -> Html {
    let CitySectionProps { title, city_names } = &props;

    html! {
        <ul class="city-ul">
            <li class="city-li"  data-cate={title}>
                {title}
            </li>
            {for city_names.iter().map(|name| {
                html!{
                    <CityItem
                        name={name}
                    />}
            })}
         </ul>
    }
}

#[functional_component]
pub fn city_selector() -> Html {
    let context = use_context::<Rc<StoreModel>>();
    let ctx = &context.unwrap();

    let StoreModel {
        city_selector_visible: show,
        ..
    } = &***ctx;

    let show1 = show.clone();

    let hidden_class = if *show { "" } else { "hidden" };

    let context_dispatch = use_context::<StoreDispatch>();
    let onclick: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch {
        Some(dispatch) => {
            let dispatch = &*dispatch;
            dispatch.emit(Action::ToggleCitySelectorVisible);
            return ();
        }
        _ => (),
    });

    let (loading, set_loading) = use_state(|| false);

    let (search_word, set_search_word) = use_state(|| "".to_string());
    let search_word = &*search_word;
    let has_search_word = search_word.len() > 0;

    let clear_hidden_class = if has_search_word { "" } else { "hidden" };

    let set_search_word = Rc::new(set_search_word);
    let set_search_word1 = Rc::clone(&set_search_word);

    let oninput: Callback<yew::html::InputData> =
        Callback::from(move |evt: yew::html::InputData| set_search_word(evt.value));

    let onclear: Callback<MouseEvent> = Callback::from(move |_| set_search_word1("".to_string()));

    let (city_data, set_city_data) = use_state(|| CityResult::new());
    let sections = &(*city_data).cityList;

    let alphabet: Vec<char> = "ABCDEFGIJKLMNOPQRSTUVWXYZ".chars().collect();

    use_effect_with_deps(
        move |_| {
            if show1 {
                set_loading(true);
                let future = async { get_city_list().await };

                handle_future(future, move |value: CityResult| {
                    set_loading(false);
                    set_city_data(value)
                });
            }

            return || ();
        },
        (show1),
    );

    html! {
        <div
        class=format!("{} {}", "city-selector" ,hidden_class )
        >
        <div class="city-search">
            <div class="search-back"
            >
            <div class="header-back"  style=" width=42 "
            onclick=onclick
            >
            {"<"}
            </div>
            </div>
            <div class="search-input-wrapper">
                <input
                    type="text"
                    value=search_word
                    class="search-input"
                    placeholder="城市、车站的中文或拼音"
                    oninput=oninput
                />
            </div>
            <div
                onclick=onclear
                class=format!("{} {}", "search-clean" ,clear_hidden_class )
            >
               { "x"}
            </div>
        </div>
        {if has_search_word {
            html! {
                <Suggest
                search_word=search_word
                />
        }} else {  html!{ <div/>} } }
        {if *loading { html! {<div>{"loading"}</div>} } else {
        html! {
            <div class="city-list">
                <div class="city-cate">
                    {for sections.iter().map(move |section| {
                        let city_names: Vec<String> = section.citys.iter().map(|city|{
                            let City {name} =city;
                            name.to_string()
                        }).collect();
                        html! {
                            <CitySection
                                title={section.title.clone()}
                                city_names=city_names
                                // onSelect={onSelect}
                            />
                        }
                    })}
                </div>
                <div class="city-index">
                    {for alphabet.iter().map(|alpha|{
                        html! {
                            <AlphaIndex
                                alpha={alpha}
                                // onClick={toAlpha}
                            />
                        }
                    })}
                </div>
            </div>
            } } }
        </div>
    }
}
