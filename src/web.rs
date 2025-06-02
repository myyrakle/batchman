#[path = "./jsbundle.rs"]
pub mod jsbundle;

pub async fn index_html() -> impl axum::response::IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>React App</title>
  </head>
  <body>
    <div id="root"></div>
    <script src="bundle.js"></script>
  </body>
</html>
"#;

    axum::response::Html(html)
}

pub async fn bundle_js() -> impl axum::response::IntoResponse {
    let js = jsbundle::JS_BUNDLE;

    axum::response::Response::builder()
        .header("Content-Type", "application/javascript")
        .body(axum::body::Body::from(js))
        .unwrap()
}
