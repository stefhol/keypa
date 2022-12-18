use std::fs;

use openapi::ApiDoc;
use utoipa::OpenApi;
pub mod api;
pub mod crud;
pub mod openapi;
pub mod util;
// in ./src/gen_openapi.rs
fn main() {
    print_to_file(&ApiDoc::openapi())
}
fn print_to_file(openapi: &utoipa::openapi::OpenApi) {
    fs::write("api/spec.json", openapi.to_pretty_json().unwrap()).unwrap();
}
