use std::rc::Rc;
use yew::{events::MouseEvent, html, Callback, Component, ComponentLink, Html, ShouldRender};
use yewtil::{effect, Effect};

pub struct Model {
    value: bool,
    val: i32,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Effect<Self>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            value: false,
            val: 0,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        msg.call(self)
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let toggle = self.link.callback(|_| {
            effect(|model: &mut Self| {
                model.value = !model.value;
                true
            })
        });

        let increment = self.link.callback(|_| {
            effect(|model: &mut Self| {
                model.val += 1;
                true
            })
        });

        html! {
            <>
                <div>
                   {self.value}
                </div>
                <div>
                    <button onclick=toggle>
                        {"Toggle"}
                    </button>
                </div>

                <div>
                   {self.val}
                </div>
                <div>
                    <button onclick=increment>
                        {"Toggle"}
                    </button>
                </div>
            </>
        }
    }
}
