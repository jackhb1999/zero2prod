use validator::ValidateEmail;
#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if (&s).validate_email() {
            Ok(SubscriberEmail(s))
        } else {
            Err(format!("invalid email: {}", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests{
    use claim::assert_ok;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use crate::domain::SubscriberEmail;

    #[test]
    fn valid_emails_are_parsed_successfully(){
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }


}