use web_view::builder;
use yew::App;

pub struct AppBuilder {
    window_name: String,
}

impl Default for AppBuilder {
    fn default() -> Self {
        todo!()
    }
}

// type WindowBuilder<'a, A, C> = Fn(
//     web_view::WebViewBuilder<'a, A, Invoker<A>, C>,
// ) -> web_view::WebViewBuilder<'a, A, Invoker<A>, C>;

// dyn Fn(web_view::WebView<()>, &str) ;
impl AppBuilder {
    pub fn builder() -> Self {
        Self::default()
    }
    pub fn register_service(mut self) -> Self {
        self
    }
    pub fn window<COMP, A, F: Fn(web_view::WebViewBuilder<'_, (), A, &str>)>(
        mut self,
        builderFn: F,
    ) -> Self
    where
        COMP: yew::Component,
        COMP::Properties: Default,
    {
        // let mut builder = web_view::builder()
        //     .content(web_view::Content::Url("abc123"))
        //     .invoke_handler(|_, _| Ok(()))
        //     .title("bla")
        //     .user_data(());

        // let d = builderFn(&mut builder);

        self
    }
    pub fn window_main<COMP, A, F: Fn(web_view::WebViewBuilder<'_, (), A, &str>)>(
        mut self,
        builderFn: F,
    ) where
        COMP: yew::Component,
        COMP::Properties: Default,
    {
    }
}

// #[test]
// fn demo_service() {
//     AppBuilder::default().window(|b| {
//         b.title("My Project").invoke_handler(|_, _| Ok(()));

//         //     .size(320, 480)
//         //     .resizable(false)
//         //     .debug(true)
//         //     .user_data(());
//     });
// }
