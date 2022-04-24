use infer;
use regex::Regex;
// DO NOT READ FILE CONTENTS INSIDE THIS FUNCTION
// TODO : specify parameter type(s) and return type(s)
pub fn validate_file(path: &str, extension: bool) -> bool {
    // TODO : implement logic
    let kind = match infer::get_from_path(path) {
        Ok(kind) => match kind {
            Some(kind) => kind,
            None => {
                return false;
            }
        },
        Err(err) => {
            println!("{}", err);
            return false;
        }
    };

    match kind.matcher_type() {
        infer::MatcherType::Image | infer::MatcherType::Video => {
            if extension {
                let extension_rule = Regex::new(r"\.[^\.]*$").unwrap();
                let exts = extension_rule.find(path).unwrap();
                kind.extension()
                    .to_string()
                    .eq(&path[exts.start() + 1..exts.end()])
            } else {
                true
            }
        }
        _ => false,
    }
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn validate_file_image_ok() {
        let path = "files/file_example_JPG_500kB.jpg";

        assert_eq!(validate_file(path, false), true);
    }

    #[test]
    fn validate_file_image_with_extension_ok() {
        let path = "files/file_example_JPG_500kB.jpg";

        assert_eq!(validate_file(path, true), true);
    }

    #[test]
    fn validate_file_image_with_wrong_extension_error() {
        let path = "files/file_example_JPG_500kB.jpeg";

        assert_eq!(validate_file(path, true), false);
    }

    #[test]
    fn validate_file_video_ok() {
        let path = "files/file_example_AVI_480_750kB.avi";

        assert_eq!(validate_file(path, false), true);
    }

    #[test]
    fn validate_file_video_with_extension_ok() {
        let path = "files/file_example_AVI_480_750kB.avi";

        assert_eq!(validate_file(path, true), true);
    }

    #[test]
    fn validate_file_image_no_extension_error() {
        let path = "files/file_example_JPG_500kB";

        assert_eq!(validate_file(path, false), false);
    }

    #[test]
    fn validate_file_not_enough_perm_error() {
        let path = "/etc/shadow";

        assert_eq!(validate_file(path, false), false);
    }

    #[test]
    fn validate_file_reject_file_type_error() {
        let path = "files/file_example_MP3_700KB.mp3";

        assert_eq!(validate_file(path, false), false);
    }
}
