use axum:: {Router, routing::get};
// our router
let app = Router::new()
    .route("/api/signup", get(root))
    .route("/api/foo", get(get_foo).post(post_foo))
    .route("/api/foo/bar", get(foo_bar));

// which calls one of these handlers
async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}