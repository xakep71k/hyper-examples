use futures::prelude::*;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3333));

    let make_service = hyper::service::make_service_fn(|_conn| async {
        let svc = Logging::new(HelloWorld);
        Ok::<_, std::convert::Infallible>(svc)
    });

    let server = hyper::Server::bind(&addr).serve(make_service);
    server.await.expect("server failed");
}

#[derive(Clone)]
struct HelloWorld;

impl tower::Service<hyper::Request<hyper::Body>> for HelloWorld {
    type Response = hyper::Response<hyper::Body>;
    type Error = std::convert::Infallible;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: hyper::Request<hyper::Body>) -> Self::Future {
        let resp = hyper::Response::new(hyper::Body::from("Hello, world!"));
        futures::future::ready(Ok(resp))
    }
}

struct Logging<S> {
    inner: S,
}

impl<S> Logging<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> tower::Service<hyper::Request<B>> for Logging<S>
where
    S: tower::Service<hyper::Request<B>> + 'static + Clone + Send,
    B: 'static + Send,
    S::Future: 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::Map<
        S::Future,
        fn(Result<Self::Response, Self::Error>) -> Result<Self::Response, Self::Error>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<B>) -> Self::Future {
        println!("START");
        self.inner.call(req).map(|result| {
            println!("END");
            result
        })
    }
}
