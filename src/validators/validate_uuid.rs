use std::fs;

use lazy_static::lazy_static;
use regex::Regex;
use uuid::Uuid;

// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
// TODO : specify parameter type(s) and return type(s)
pub fn validate_uuid(uuid: &str) -> bool {
    // TODO : implement logic

    let uuid = Uuid::try_parse(uuid);
    match uuid {
        Ok(_) => (),
        Err(_) => return false,
    }
    lazy_static! {
        static ref UUID_RULE: Regex = Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$"
        )
        .unwrap();
    }

    UUID_RULE.is_match(&uuid.unwrap().hyphenated().to_string())
}

pub fn validate_file_uuid(path: &str, uuid: &str) -> bool {
    let file_data = fs::read(path).unwrap();
    let buffer = file_data.as_slice();
    let file_uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, buffer);
    let file_uuid = file_uuid.to_string();

    file_uuid.eq(uuid)
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn validate_uuid_format_hyphenated_pass() {
        let uuid = "b79cb3ba-745e-5d9a-8903-4a02327a7e09";

        assert_eq!(validate_uuid(uuid), true);
    }

    #[test]
    fn validate_uuid_format_simple_pass() {
        let uuid = "b79cb3ba745e5d9a89034a02327a7e09";

        assert_eq!(validate_uuid(uuid), true);
    }
}
