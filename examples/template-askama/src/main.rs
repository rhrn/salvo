use salvo::prelude::*;
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")] 
struct HelloTemplate<'a> {
    name: &'a str,
}

#[handler]
async fn hello_world(req: &mut Request, res: &mut Response) { 
    let hello = HelloTemplate { name: req.param::<&str>("name").unwrap_or("World") };
    res.render(Text::Html(hello.render().unwrap()));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Listening on http://127.0.0.1:7878");
    let router = Router::with_path("<name>").get(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
