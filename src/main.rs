use clap::Parser;
use open;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "ADO PRs right from the terminal",
    author = "Krithik Suvarna <krithiksuvarna@gmail.com>",
    help_template = "{about}{author-section}\n\n{usage-heading}\n{tab}{usage}\n\n{all-args}"
)]
struct Args {
    repo: String,
    #[arg(short, default_value_t=String::from("main"))]
    target_branch: String,
    #[arg(short)]
    source_branch: String,
    #[arg(short, default_value_t=String::from("YourOrg"))]
    organization: String,
    #[arg(short)]
    browser: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let url = format!("https://dev.azure.com/Company/{org}/_git/{repo}/pullrequestcreate?_a=overview&sourceRef={source}&targetRef={target}",
        org = args.organization,
        repo = args.repo,
        source = args.source_branch,
        target = args.target_branch
    );
    match args.browser {
        Some(browser) => match browser.to_lowercase().as_str() {
            "chrome" => open::with(url, "google chrome")?,
            _ => open::with(url, browser)?,
        },
        None => open::that(url)?,
    }
    Ok(())
}
