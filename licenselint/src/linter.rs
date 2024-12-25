use crate::config::Config;
use crate::issue::Issue;
use crate::license::License;
use crate::template::arkts_apache20::ArktsApache20Template;
use crate::template::clang_format_apache20::ClangFormatApache20Template;
use crate::template::cmake_apache20::CmakeApache20Template;
use crate::template::cmake_lists_apache20::CmakeListsApache20Template;
use crate::template::cpp_apache20::CppApache20Template;
use crate::template::go_apache20::GoApache20Template;
use crate::template::hpp_apache20::HppApache20Template;
use crate::template::in_apache20::InApache20Template;
use crate::template::ipp_apache20::IppApache20Template;
use crate::template::java_apache20::JavaApache20Template;
use crate::template::properties_apache20::PropertiesApache20Template;
use crate::template::python_apache20::PythonApache20Template;
use crate::template::rust_apache20::RustApache20Template;
use crate::template::toml_apache20::TomlApache20Template;
use crate::template::tpp_apache20::TppApache20Template;
use crate::template::typescript_apache20::TypeScriptApache20Template;
use crate::template::xml_apache20::XmlApache20Template;
use crate::template::yaml_apache20::YamlApache20Template;
use crate::template::LintTemplate;
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

pub struct Linter<'a> {
    config: &'a Config,
    templates: HashMap<String, Box<dyn LintTemplate>>,
    exact_match_templates: HashMap<String, Box<dyn LintTemplate>>,
}

impl<'a> Linter<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut linter = Linter {
            config,
            templates: HashMap::new(),
            exact_match_templates: HashMap::new(),
        };

        linter.init_templates_by_license();

        linter
    }

    fn init_templates_by_license(&mut self) {
        match self.config.license {
            License::Apache20 => {
                self.add_exact_template(".clang-format", ClangFormatApache20Template {});
                self.add_exact_template("CMakeLists.txt", CmakeListsApache20Template {});

                self.add_template("ets", ArktsApache20Template {});
                self.add_template("cmake", CmakeApache20Template {});
                self.add_template("cpp", CppApache20Template {});
                self.add_template("go", GoApache20Template {});
                self.add_template("hpp", HppApache20Template {});
                self.add_template("in", InApache20Template {});
                self.add_template("ipp", IppApache20Template {});
                self.add_template("java", JavaApache20Template {});
                self.add_template("properties", PropertiesApache20Template {});
                self.add_template("py", PythonApache20Template {});
                self.add_template("rs", RustApache20Template {});
                self.add_template("toml", TomlApache20Template {});
                self.add_template("tpp", TppApache20Template {});
                self.add_template("ts", TypeScriptApache20Template {});
                self.add_template("xml", XmlApache20Template {});
                self.add_template("yaml", YamlApache20Template {});
                self.add_template("yml", YamlApache20Template {});
            }
        }
    }

    /// Add a template for a specific file extension.
    pub fn add_template<T: LintTemplate + 'static>(&mut self, extension: &str, template: T) {
        self.templates
            .insert(extension.to_string(), Box::new(template));
    }

    /// Add an exact match template for specific filenames (like .clang-format).
    pub fn add_exact_template<T: LintTemplate + 'static>(&mut self, filename: &str, template: T) {
        self.exact_match_templates
            .insert(filename.to_string(), Box::new(template));
    }

    pub fn check_files_in_dir(
        &self,
        dir: &Path,
    ) -> Result<Vec<Issue>, Vec<(std::path::PathBuf, io::Error)>> {
        let mut all_issues = Vec::new();

        let result = self.travel_dir(dir, |path| match fs::read_to_string(path) {
            Ok(content) => {
                let issues = self.check(path.to_str().unwrap(), &content);
                all_issues.extend(issues);
                Ok(())
            }
            Err(e) => Err(e),
        });

        result.map(|_| all_issues)
    }

    pub fn format_files_in_dir(
        &self,
        dir: &Path,
    ) -> Result<(), Vec<(std::path::PathBuf, io::Error)>> {
        self.travel_dir(dir, |path| match fs::read_to_string(path) {
            Ok(content) => {
                let formatted_content = self.format(path.to_str().unwrap(), &content);
                if formatted_content != content {
                    fs::write(path, formatted_content)
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        })
    }

    fn travel_dir<F>(
        &self,
        dir: &Path,
        mut file_handler: F,
    ) -> Result<(), Vec<(std::path::PathBuf, io::Error)>>
    where
        F: FnMut(&Path) -> Result<(), io::Error>,
    {
        let mut errors = Vec::new();
        let mut builder = WalkBuilder::new(dir);

        builder
            .ignore(false)
            .hidden(false)
            .follow_links(true)
            .parents(true)
            .require_git(false)
            .git_exclude(true)
            .git_global(true)
            .git_ignore(true);

        builder.add_ignore(".licenselintignore");

        let walker = builder.build();

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if entry
                        .path()
                        .components()
                        .any(|comp| comp.as_os_str() == ".git")
                    {
                        continue;
                    }

                    if entry.path().file_name() == Some(std::ffi::OsStr::new(".gitmodules")) {
                        continue;
                    }

                    let ignored_extensions = ["md", "png", "xlsx", "xlss"];

                    if entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map_or(false, |ext| ignored_extensions.contains(&ext))
                    {
                        continue;
                    }

                    if let Ok(content) = std::fs::read(entry.path()) {
                        if std::str::from_utf8(&content).is_err() {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        if let Err(e) = file_handler(entry.path()) {
                            errors.push((entry.path().to_path_buf(), e));
                        }
                    }
                }
                Err(e) => errors.push((dir.to_path_buf(), io::Error::new(io::ErrorKind::Other, e))),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn check(&self, filename: &str, content: &str) -> Vec<Issue> {
        let mut all_issues = Vec::new();

        let path = Path::new(filename);

        // Check for exact match templates
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if let Some(template) = self.exact_match_templates.get(file_name) {
                let issues = template.check(self.config, filename, content);
                all_issues.extend(issues);
            }
        }

        // Check for extension-based templates if no exact match
        if all_issues.is_empty() {
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                if let Some(template) = self.templates.get(extension) {
                    let issues = template.check(self.config, filename, content);
                    all_issues.extend(issues);
                }
            }
        }

        all_issues
    }

    pub fn format(&self, filename: &str, content: &str) -> String {
        let path = Path::new(filename);

        // Check for exact match templates
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if let Some(template) = self.exact_match_templates.get(file_name) {
                return template.format(self.config, filename, content);
            }
        }

        // Check for extension-based templates
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            if let Some(template) = self.templates.get(extension) {
                return template.format(self.config, filename, content);
            }
        }

        content.to_string()
    }
}
