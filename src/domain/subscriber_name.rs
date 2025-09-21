use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            // panic!("{}", format!("{} is not a valid subscriber name", s));
            Err(format!("{} is not a valid subscriber name", s))
        } else {
            Ok(SubscriberName(s))
        }
    }

    // 值暴露
    pub fn inner(self) -> String {
        self.0
    }
    // 暴露可变引用
    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }
    // 共享引用
    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SubscriberName {
    // 共享引用的 AsRef 实现
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::assert_ok;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a`".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_then_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Jack H B Fan".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}