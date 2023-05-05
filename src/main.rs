use clap::Parser;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::error::Error;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'd', long = "domain")]
    domain: Option<String>,
    #[arg(short = 'n', long = "name")]
    name: Option<String>,
    #[arg(short = 'p', long = "path")]
    path: Option<String>,
    #[arg(short = 'P', long = "png")]
    png: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::parse();
    let browser: Browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let pdfurl: String;
    let pdf;
    let pdfname: String;
    let pngname: String;

    if args.name.is_some() {
        pdfname = format!("{:?}.pdf", &args.name.clone().expect("no name provided"));
        pngname = format!("{:?}.png", &args.name.clone().expect("no name provided"));
    } else {
        pdfname = String::from("rust.pdf");
        pngname = String::from("rust.png");
    }

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
        fs::write(&pdfname, &pdf)?;
    };

    if args.png && (args.path.is_some() || args.domain.is_some()) {
        let png = tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Png,
            Some(75),
            None,
            true,
        )?;
        fs::write(&pngname, png)?;
    };

    Ok(())
}
