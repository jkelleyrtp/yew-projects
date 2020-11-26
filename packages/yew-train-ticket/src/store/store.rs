use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct StoreModel {
    pub from: String,
    pub to: String,
    pub local_time: DateTime<Local>,
    pub is_high_speed: bool,
    pub date_selector_visible: bool,
    pub city_selector_visible: bool,
    pub is_selecting_from: bool,
}

pub enum Action {
    ExchangeFromTo,
    ToggleHighSpeed,
    ToggleDateSelectorVisible,
    ToggleCitySelectorVisible,
    SelectDate(DateTime<Local>),
    SelectFrom(String),
    SelectTo(String),
    SetIsSelectingFrom(bool),
}

#[derive(Clone)]
pub struct StoreDispatch(pub std::rc::Rc<dyn std::ops::Fn(Action) -> ()>);

impl PartialEq for StoreDispatch {
    fn eq(&self, other: &StoreDispatch) -> bool {
        false
    }
}
impl PartialEq for StoreModel {
    fn eq(&self, other: &StoreModel) -> bool {
        false
    }
}

impl StoreDispatch {
    pub fn emit(&self, action: Action) -> () {
        (self.0)(action);
    }
}

pub fn reducer(prev: std::rc::Rc<StoreModel>, action: Action) -> StoreModel {
    let StoreModel {
        is_high_speed,
        to,
        from,
        date_selector_visible,
        city_selector_visible,
        ..
    } = &*prev;
    match action {
        Action::ExchangeFromTo => StoreModel {
            to: from.clone(),
            from: to.clone(),
            ..*prev
        },
        Action::ToggleHighSpeed => StoreModel {
            from: from.clone(),
            to: to.clone(),
            is_high_speed: !*is_high_speed,
            ..*prev
        },
        Action::ToggleDateSelectorVisible => StoreModel {
            from: from.clone(),
            to: to.clone(),
            date_selector_visible: !*date_selector_visible,
            ..*prev
        },
        Action::ToggleCitySelectorVisible => StoreModel {
            from: from.clone(),
            to: to.clone(),
            city_selector_visible: !*city_selector_visible,
            ..*prev
        },
        Action::SelectDate(date) => StoreModel {
            from: from.clone(),
            to: to.clone(),
            local_time: date,
            ..*prev
        },
        Action::SelectFrom(from) => StoreModel {
            from,
            to: to.clone(),
            ..*prev
        },
        Action::SelectTo(to) => StoreModel {
            from: from.clone(),
            to,
            ..*prev
        },
        Action::SetIsSelectingFrom(is_selecting_from) => StoreModel {
            from: from.clone(),
            to: to.clone(),
            is_selecting_from,
            ..*prev
        },
    }
}
