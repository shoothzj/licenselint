use crate::config::Config;
use crate::issue::Issue;
use crate::template::LintTemplate;
use regex::Regex;

pub struct XmlApache20Template;

impl XmlApache20Template {
    const TEMPLATE: &'static str = r#"<!--
    Copyright {year} {author}

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->"#;

    const XML_HEADER: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
}

impl LintTemplate for XmlApache20Template {
    fn check(&self, config: &Config, filename: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let escaped_template = regex::escape(&format!("{}\n{}", Self::XML_HEADER, Self::TEMPLATE));

        let expected_license = escaped_template
            .replace(r"\{author\}", &regex::escape(&config.formatted_author))
            .replace(r"\{year\}", r"\d{4}");

        let re = Regex::new(&format!(r"(?m)^{}", expected_license)).unwrap();

        if !re.is_match(content) {
            issues.push(Issue::new(filename));
        }

        issues
    }

    fn format(&self, config: &Config, _filename: &str, content: &str) -> String {
        let license_text = format!(
            "{}",
            Self::TEMPLATE
                .replace("{year}", &config.formatted_year)
                .replace("{author}", &config.formatted_author)
        );

        let mut lines: Vec<&str> = content.lines().collect();

        if lines.get(0) == Some(&Self::XML_HEADER) {
            lines.remove(0);
        }

        let remaining_content = lines.join("\n");

        if remaining_content.starts_with(&license_text) {
            return format!("{}\n{}\n", Self::XML_HEADER, remaining_content);
        }

        format!(
            "{}\n{}\n\n{}\n",
            Self::XML_HEADER,
            license_text,
            remaining_content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::license::License;

    #[test]
    fn test_check_valid_license() {
        let template = XmlApache20Template;
        let config = Config::new_from_author(
            License::Apache20,
            "ZhangJian He".to_string(),
            "1997".to_string(),
        );

        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!--
    Copyright 1997 ZhangJian He

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->"#;

        let issues = template.check(&config, "main.xml", content);
        assert!(
            issues.is_empty(),
            "There should be no issues with a valid license"
        );
    }

    #[test]
    fn test_check_invalid_license() {
        let template = XmlApache20Template;
        let config = Config::new_from_author(
            License::Apache20,
            "ZhangJian He".to_string(),
            "1997".to_string(),
        );

        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!--
    Copyright 1997 ZhangJian He

    Licensed under the XYZ License.
-->
"#;

        let issues = template.check(&config, "main.xml", content);
        assert_eq!(
            issues.len(),
            1,
            "There should be one issue for an invalid license"
        );
    }

    #[test]
    fn test_format_no_license() {
        let template = XmlApache20Template;
        let config = Config::new_from_author(
            License::Apache20,
            "ZhangJian He".to_string(),
            "1997".to_string(),
        );

        let content = r#"int main() {
    return 0;
}
"#;

        let expected = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!--
    Copyright 1997 ZhangJian He

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->

int main() {{
    return 0;
}}
"#
        );

        let formatted_content = template.format(&config, "main.xml", content);
        assert_eq!(
            formatted_content, expected,
            "The license should be added to the content"
        );
    }

    #[test]
    fn test_format_with_license() {
        let template = XmlApache20Template;
        let config = Config::new_from_author(
            License::Apache20,
            "ZhangJian He".to_string(),
            "1997".to_string(),
        );

        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!--
    Copyright 1997 ZhangJian He

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->

int main() {
    return 0;
}
"#;

        let formatted_content = template.format(&config, "main.xml", content);
        assert_eq!(
            formatted_content, content,
            "Content should remain unchanged if the license is already present"
        );
    }
}
