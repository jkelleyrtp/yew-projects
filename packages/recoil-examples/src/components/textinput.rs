use yew::prelude::*;
use yew_functional::function_component;
use yew_recoil::hooks::use_recoil_state;

use crate::state::TITLE;

#[function_component(TextBox)]
pub fn text_box() -> Html {
    let (title, set_title) = use_recoil_state(&TITLE);

    let oninput = Callback::from(move |e: InputData| set_title(e.value));

    if true {
        html! {
        <div>
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                </div>
                <input
                    type="text"
                    id="price"
                    class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md"
                    placeholder="0.00"
                    value=&title
                    oninput=oninput
                />
                <div class="absolute inset-y-0 right-0 flex items-center">
                <label for="currency" class="sr-only">{"Currency"}</label>
            </div>
            {vec![
                use_template(include_str!("../templates/intro.html")),
                use_template(include_str!("../templates/flier.html")),
            ]}
        </div>
        }
    } else {
        html! {
            <div>
                <label for="price" class="block text-sm font-medium text-gray-700">{"Price"}</label>
                <label for="price" class="block text-sm font-medium text-gray-700">{"Price"}</label>
                <label for="price" class="block text-sm font-medium text-gray-700">{"Price"}</label>
                <label for="price" class="block text-sm font-medium text-gray-700">{"Price"}</label>
                <div class="mt-1 relative rounded-md shadow-sm">
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        </div>
                        <input
                            type="text"
                            id="price"
                            class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md"
                            placeholder="0.00"
                            value=&title
                            oninput=oninput
                        />
                        <div class="absolute inset-y-0 right-0 flex items-center">
                        <label for="currency" class="sr-only">{"Currency"}</label>
                    </div>
                </div>
            </div>
        }
    }
}

fn use_template(src: &'static str) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();
    div.set_inner_html(src);
    Html::VRef(div.into())
}
