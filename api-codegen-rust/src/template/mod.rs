use handlebars::{to_json, Handlebars};
use serde::Serialize;

pub static METHOD: &'static str = include_str!("method.rs.hbs");
pub static MODULE: &'static str = include_str!("module.rs.hbs");
pub static LIB: &'static str = include_str!("lib.rs.hbs");

#[derive(Serialize)]
pub struct MethodModel {
    pub http_paths: String,
    pub http_method: String,
    pub method_name: String,
    pub method_return: String,
    pub arguments: String,
    pub has_body: bool,
    pub has_query: bool,
}

#[derive(Serialize)]
pub struct ModuleModel {
    pub module_name: String,
    pub user_types: Vec<String>,
    pub methods: Vec<String>,
}

pub fn render<T: Serialize>(templ: &str, model: &T) -> String {
    let hbs = Handlebars::new();
    hbs.render_template(templ, &to_json(model)).unwrap()
}

pub fn render_method(model: &MethodModel) -> String {
    render(METHOD, model)
}

pub fn render_module(model: &ModuleModel) -> String {
    render(MODULE, model)
}
