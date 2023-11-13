use clap::Parser;

/// Http health checker.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(help_template = "\n\
    \n----------------------------------------------------------------------\
    \n\n{author-with-newline}\
    Version: {version}
    \n{about-section} {usage-heading} {usage} \
    \n\n {all-args} {tab}\
    ")]
pub struct CliHttpHealthChecker {
    /// Specifies timeout between requests in seconds.
    #[arg(short = 'F', value_name = "FREQUENCY")]
    pub freq: usize,
    /// Specifies Url to call.
    #[arg(short = 'U', value_name = "URL")]
    pub url: String,
}
