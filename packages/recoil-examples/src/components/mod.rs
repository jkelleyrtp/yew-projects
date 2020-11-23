use recoil_yew::prelude::RecoilRoot;
use yew::prelude::*;
use yew_functional::*;

mod canvas;
mod counter;
mod display;
mod textinput;

#[function_component(MyApp)]
pub fn my_app() -> Html {
    html! {
        <RecoilRoot >
            <link href="https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css" rel="stylesheet" />

            <div class="mt-10 flex flex-col m-20 h-screen">
                <display::Display />
                <canvas::Canvas />
            </div>
        </RecoilRoot >
    }
}

// <div class="mt-10 mx-auto max-w-7xl px-4 sm:mt-12 sm:px-6 md:mt-16 lg:mt-20 lg:px-8 xl:mt-28">
