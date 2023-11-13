use clap::Parser;
use http_health_checker::app::cli::layout::CliHttpHealthChecker;
use url::Url;
use url::UrlParser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = CliHttpHealthChecker::parse();
    let timeout = cli.freq;
    let url: &str = &cli.url;

    let _url_check = Url::parse(url);
    if _url_check.is_err() {
        println!("URL parsing error")
    }

    while _url_check.is_ok() {
        print!("Checking '{url}'.");
        let resp = reqwest::get(url).await.unwrap();
        if resp.status().is_success() {
            print!("Result: OK(200)\n");
        } else {
            print!("Result: ERR({})\n", resp.status().as_u16());
        }

        tokio::time::sleep(std::time::Duration::from_secs(timeout as _)).await;
    }
}
