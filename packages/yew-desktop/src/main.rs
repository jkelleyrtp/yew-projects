use web_view::*;

fn main() {
    colog::init();

    std::thread::spawn(|| async_std::task::block_on(webserver()));

    web_view::builder()
        .title("My Project")
        .content(Content::Url("http://127.0.0.1:9000"))
        .size(1200, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

async fn webserver() -> anyhow::Result<()> {
    let mut app = tide::new();
    app.at("/")
        .get(|_| async {
            log::info!("Connected to development server");
            Ok(tide::Body::from_file("public/index.html").await?)
        })
        .serve_dir("public/")?;

    app.listen("127.0.0.1:9000").await?;

    Ok(())
}
