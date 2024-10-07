use crate::config::Config;
use crate::issue::Issue;

pub mod clang_format_apache20;
pub mod cpp_apache20;
pub mod go_apache20;
pub mod hpp_apache20;
pub mod in_apache20;
pub mod ipp_apache20;
pub mod java_apache20;
pub mod python_apache20;
pub mod rust_apache20;
pub mod toml_apache20;
pub mod tpp_apache20;
pub mod yaml_apache20;

pub trait LintTemplate {
    fn check(&self, config: &Config, filename: &str, content: &str) -> Vec<Issue>;
    fn format(&self, config: &Config, filename: &str, content: &str) -> String;
}
