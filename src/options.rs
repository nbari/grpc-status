use clap::{App, Arg};

#[must_use]
pub fn new() -> String {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("endpoint")
                .help("https://10.0.1.2:8443")
                .min_values(1)
                .required(true),
        )
        .get_matches();

    matches.value_of("endpoint").unwrap().into()
}
