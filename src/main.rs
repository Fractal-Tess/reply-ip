use axum::{
    http::{header, HeaderMap, StatusCode},
    response::Response,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|headers: HeaderMap| async move {
            let ip = headers.get("x-forwarded-for").map(|val| val.to_owned());

            let mut res_headers = HeaderMap::new();
            res_headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

            match ip {
                Some(ip) => {
                    let body = ip.to_str().unwrap().to_owned();
                    let mut res = Response::new(body);
                    *res.headers_mut() = res_headers;
                    res
                }
                None => {
                    let mut res = Response::new("Bad request".to_owned());
                    *res.status_mut() = StatusCode::BAD_REQUEST;
                    *res.headers_mut() = res_headers;
                    res
                }
            }
        }),
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
