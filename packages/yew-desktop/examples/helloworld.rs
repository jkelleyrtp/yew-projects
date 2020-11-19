use yew_desktop::launch::AppBuilder;
use yew_functional::function_component;

fn main() {
    AppBuilder::builder()
        .register_service()
        .window_main::<MyApp, _, _>(|b| {
            b.title("My Project")
                .invoke_handler(|_, _| Ok(()))
                .size(320, 480)
                .resizable(false)
                .debug(true)
                .user_data(());
        });
}

#[function_component(MyApp)]
fn my_app() -> Html {
    yew::html! {
        <div>
            {"Hello world!"}
        </div>
    }
}

/*
build.rs

use services::BlahXYZ;
use app::MyApp;

fn main() {
    AppBuilder::builder()
        .register_service()
        .window_main::<MyApp, _, _>(|b| {
            b.title("My Project")
                .invoke_handler(|_, _| Ok(()))
                .size(320, 480)
                .resizable(false)
                .debug(true)
                .user_data(());
        });
}


*/
