use crate::config::Config;
use crate::issue::Issue;
use crate::template::LintTemplate;
use regex::Regex;

pub struct ClangFormatApache20Template;

impl ClangFormatApache20Template {
    const TEMPLATE: &'static str = r#"# Copyright: {year} {author}
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
    "#;
}

impl LintTemplate for ClangFormatApache20Template {
    fn check(&self, config: &Config, filename: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let expected_license = Self::TEMPLATE.replace("{author}", &config.formatted_author);

        let license_regex = expected_license.replace("{year}", r"\d{4}");
        let re = Regex::new(&format!(r"(?m)^{}", license_regex)).unwrap();

        if !re.is_match(content) {
            issues.push(Issue::new(filename));
        }

        issues
    }

    fn format(&self, config: &Config, _filename: &str, content: &str) -> String {
        let license_text = Self::TEMPLATE
            .replace("{year}", &config.formatted_year)
            .replace("{author}", &config.formatted_author);

        if content.starts_with(&license_text) {
            return content.to_string();
        }

        format!("{}\n{}", license_text, content)
    }
}
