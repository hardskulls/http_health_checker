use clap::Parser;
use http_health_checker::app::cli::errors::UrlParseErr;
use http_health_checker::app::cli::functionality::url_health_check::check_url_in_loop;
use http_health_checker::app::cli::layout::CliHttpHealthChecker;
use std::time::Duration;
use url::Url;

fn main() {
    let cli = CliHttpHealthChecker::parse();
    let timeout = Duration::from_secs(cli.timeout as _);
    let url: &str = &cli.url;

    match Url::parse(url).is_ok() {
        true => check_url_in_loop(url, timeout),
        false => println!("Error: {UrlParseErr}"),
    }
}
