use clap::Parser;
use dirs;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(author="Kamal Kumar <iamkamalkumar@proton.me>", version, about="Convert any (local/online) website to pdf.", long_about = None, next_line_help = true)]
struct Cli {
    #[arg(short = 'd', long = "domain", group = "url")]
    domain: Option<String>,
    #[arg(short = 'n', long = "name", default_value_t = String::from("rust"))]
    name: String,
    #[arg(short = 'p', long = "path", group = "url")]
    path: Option<String>,
    #[arg(short = 'P', long = "png", requires = "url")]
    png: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::parse();
    let browser: Browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let pdfurl: String;
    let pdf;
    let mut path = dirs::home_dir().expect("cannot find path to home directory");
    path.push("Downloads");
    path.push(format!("{}.pdf", &args.name));

    if args.domain.is_some() {
        pdfurl = format!(
            "https://{}",
            &args.domain.clone().expect("no domain provided")
        );
        tab.navigate_to(&pdfurl)?.wait_until_navigated()?;
    } else if args.path.is_some() {
        pdfurl = format!("file://{}", &args.path.clone().expect("no path provided"));
        tab.navigate_to(&pdfurl)?.wait_until_navigated()?;
    } else {
        println!("no domain or path provided. See -h for help.")
    }

    pdf = tab.print_to_pdf(None)?;
    if args.path.is_some() || args.domain.is_some() {
        fs::write(&path, &pdf)?;
    };

    path.pop();
    path.push(format!("{}.png", &args.name));

    if args.png && (args.path.is_some() || args.domain.is_some()) {
        let png = tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Png,
            Some(75),
            None,
            true,
        )?;
        fs::write(&path, png)?;
    };

    Ok(())
}
