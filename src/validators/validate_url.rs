use regex::Regex;

// TODO : specify parameter type(s) and return type(s)
pub fn validate_url(url: &str, whitelist: Option<&Vec<&str>>) -> bool {
    // TODO : implement logic

    let rule = r"^((((ht|f)tp(s?)))|([a-zA-Z0-9])*://)?(www.|[a-zA-Z].)?[a-zA-Z0-9\-\.]+";
    let tld_rule = match whitelist {
        Some(tlds) => format!(r"({})",tlds.join("|")),
        None => r"\.((com|edu|gov|mil|net|org|biz|info|name|museum|us|ca|uk|co.uk|ch|fr)|[a-zA-Z\.]{3,}[a-zA-Z])".to_string()
    };
    let post_url = r"((/|#).*)*$";

    let full_rule = format!("{}{}{}", rule, tld_rule, post_url);
    println!("{}", full_rule);
    let re = Regex::new(&full_rule).unwrap();
    re.is_match(url)
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn validate_url_without_whitelist_pass() {
        let urls = [
            "http://www.example.com",
            "http://www.example..com",
            "https://www.example.com",
            "ftp://www.example.co.uk",
            "ftps://www.example.com",
            "www.example.com",
            "example.com",
            "a://b..eeee.c",
            "b..eeee.c/home.php",
        ];

        for url in urls {
            let res = validate_url(url, None);
            if !res {
                println!("{}", url);
            }
            assert_eq!(res, true);
        }
    }

    #[test]
    fn validate_url_with_whitelist_pass() {
        let urls = ["http://www.example.ch", "http://www.example.fr"];

        let whitelist = vec![".ch", ".fr"];

        for url in urls {
            let res = validate_url(url, Some(&whitelist));
            assert_eq!(res, true);
        }
    }

    #[test]
    #[should_panic]
    fn validate_url_with_whitelist_not_pass() {
        let urls = ["http://www.example.com", "http://www.example.uk"];

        let whitelist = vec![".ch", ".fr"];

        for url in urls {
            let res = validate_url(url, Some(&whitelist));
            assert_eq!(res, true);
        }
    }
}
