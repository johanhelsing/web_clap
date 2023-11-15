use clap::Parser;
use serde::Deserialize;
use std::ffi::OsString;

/// Extension trait for a clap parser that can be used on both web (through
/// query args) and native
pub trait WebParser {
    /// Cross platform version of [`clap::Parser::parse`]
    fn platform_parse() -> Self;
    /// Parse a query string as args
    #[cfg(any(target_arch = "wasm32", test))]
    fn from_query(query: &str) -> Self;
    /// Parses as if no args where provided
    fn from_no_args() -> Self;
}

impl<A: Parser + for<'a> Deserialize<'a>> WebParser for A {
    fn platform_parse() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let qs = web_sys::window()
                .unwrap()
                .location()
                .search()
                .unwrap()
                .trim_start_matches("?")
                .to_owned();

            let js = qs.clone().into();
            web_sys::console::log_1(&js);

            Self::from_query(&qs)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match Self::try_parse() {
                Ok(args) => args,
                Err(err) => {
                    let args: Vec<_> = std::env::args().collect();
                    panic!("Failed to parse command line args: {err}\n{args:?}");
                }
            }
        }
    }

    #[cfg(any(target_arch = "wasm32", test))]
    fn from_query(query: &str) -> Self {
        // todo: perhaps better to use parse_from?
        serde_qs::from_str(query).unwrap()
    }

    fn from_no_args() -> Self {
        let args = Vec::<OsString>::new();
        Self::parse_from(args)
    }
}

#[cfg(test)]
mod test {
    use crate::WebParser;
    use clap::Parser;
    use serde::Deserialize;

    #[derive(Parser, Debug, Clone, Deserialize)]
    #[serde(default)]
    pub struct Args {
        #[clap(short, long, default_value = "123")]
        pub test: String,
    }

    // TODO: derive this instead?
    impl Default for Args {
        fn default() -> Self {
            Self::from_no_args()
        }
    }

    #[test]
    fn default() {
        let args = Args::default();
        assert_eq!(args.test, "123");
    }

    #[test]
    fn from_empty_query_string() {
        let args = Args::from_query("");
        assert_eq!(args.test, "123");
    }

    #[test]
    fn from_query_string() {
        let args = Args::from_query("");
        assert_eq!(args.test, "123");
    }
}
