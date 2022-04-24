use lazy_static::lazy_static;
use regex::Regex;

// TODO : specify parameter type(s) and return type(s)
pub fn validate_url(url: &str, whitelist: Option<&Vec<&str>>) -> bool {
    // TODO : implement logic

    lazy_static! {
        static ref TDL_RULE: Regex = Regex::new(r"^\.([a-zA-Z\.]{1,}[a-zA-Z])$").unwrap();
    }

    let starting_rule =
        r"^((((ht|f)tp(s?)))|([a-zA-Z0-9])*://)?(www\.|[a-zA-Z\.])?[a-zA-Z0-9\-\.]+";
    let tld_rule = match whitelist {
        Some(tlds) => {
            for tld in tlds {
                if !TDL_RULE.is_match(tld) {
                    return false;
                }
            }
            format!(r"({})", tlds.join("|"))
        }
        None => r"\.([a-zA-Z\.]{1,}[a-zA-Z])".to_string(),
    };
    let post_url = r"((/|#).*)*$";

    let full_rule = format!("{}{}{}", starting_rule, tld_rule, post_url);
    println!("{}", full_rule);
    let regex_url_rule = Regex::new(&full_rule).unwrap();
    regex_url_rule.is_match(url)
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::*;
    use rstest::rstest;

    #[rstest(
        url,
        case("http://www.example.com"),
        case("http://wwrd.example.com"),
        case("http://www.example..com"),
        case("https://www.example.com"),
        case("ftp://www.example.co.uk"),
        case("ftps://www.example.com"),
        case("www.example.com"),
        case("example.com"),
        case("a://b..eeee.c"), // Ok d'accord, j'abuse peut-être un peu
        case("b..eeee.c/home.php"),
        case("http://l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.l.org"),
        case("http://tech.yahoo.com/rc/desktops/102;_ylt=Ao8yevQHlZ4On0O3ZJGXLEQFLZA5"),
    )]
    fn validate_url_pass(url: &str) {
        let res = validate_url(url, None);
        assert_eq!(res, true);
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
    // #[should_panic]
    fn validate_url_with_whitelist_not_pass() {
        let urls = ["http://www.example.com", "http://www.example.uk"];

        let whitelist = vec![".ch", ".fr"];

        for url in urls {
            let res = validate_url(url, Some(&whitelist));
            assert_eq!(res, false);
        }
    }

    #[test]
    #[should_panic]
    fn validate_url_with_whitelist_wrong_not_pass() {
        let urls = ["http://www.example.com", "http://www.example.uk"];

        let whitelist = vec![".h", ".fr"];

        for url in urls {
            let res = validate_url(url, Some(&whitelist));
            assert_eq!(res, true);
        }
    }
}
