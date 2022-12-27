use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // CachingHeader must be before Compression.
    let router = Router::with_hoop(CachingHeaders::new())
        .hoop(Compression::new().with_min_length(0))
        .push(Router::with_path("/server").get(hello))
        .push(Router::with_path("<**rest>").handle(spa()));

    let acceptor = TcpListener::bind("0.0.0.0:7878");
    Server::new(acceptor).serve(router).await;
}

fn spa() -> salvo::proxy::Proxy<Vec<&'static str>> {
    salvo::proxy::Proxy::new(vec!["http://localhost:3000"])
}
