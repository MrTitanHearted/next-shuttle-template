use std::path::PathBuf;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] ref static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service(
        "/",
        ServeDir::new(static_folder).not_found_service(ServeFile::new(format!(
            "{}/index.html",
            static_folder.to_str().unwrap_or("./static")
        ))),
    );

    Ok(router.into())
}
