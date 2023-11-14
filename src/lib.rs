use clap::Parser;
use serde::Deserialize;
use std::ffi::OsString;

#[derive(Parser, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Args {
    #[clap(short, long, default_value = "123")]
    pub test: String,
}

impl Default for Args {
    fn default() -> Self {
        let args = Vec::<OsString>::new();
        Args::parse_from(args)
    }
}

impl Args {
    pub fn get() -> Self {
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

            Args::from_query(&qs)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match Args::try_parse() {
                Ok(args) => args,
                Err(err) => {
                    let args: Vec<_> = std::env::args().collect();
                    let message = format!("Failed to parse command line args: {err}\n{args:?}");
                    if let Err(write_err) = std::fs::write("stderr.log", message) {
                        eprintln!("Failed to log to file: {write_err}");
                    }
                    err.exit();
                }
            }
        }
    }

    #[cfg(any(target_arch = "wasm32", test))]
    fn from_query(query: &str) -> Self {
        // TODO: result?
        serde_qs::from_str(query).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Args;

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
}
