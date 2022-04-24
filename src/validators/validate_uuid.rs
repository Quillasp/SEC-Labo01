use lazy_static::lazy_static;
use regex::Regex;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
// TODO : specify parameter type(s) and return type(s)
pub fn validate_uuid(uuid: &str) -> bool {
    // TODO : implement logic

    lazy_static! {
        static ref UUID_RULE: Regex = Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$"
        )
        .unwrap();
    }

    if !UUID_RULE.is_match(uuid) {
        return false;
    }
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
