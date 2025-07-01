/*
rust_selenium_web_scraping

https://brightdata.com/blog/how-tos/web-scraping-with-rust
https://www.zenrows.com/blog/rust-web-scraping
https://www.scrapingbee.com/blog/web-scraping-rust/
https://www.scrapingdog.com/blog/web-scraping-with-rust/
https://rayobyte.com/blog/rust-web-scraping/
https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
https://scrape.do/blog/web-scraping-in-rust/
https://proxiesapi.com/articles/scraping-all-the-images-from-a-website-with-rust
https://docs.rs/webbrowser/latest/webbrowser/

https://skolo.online/documents/webscrapping/
https://github.com/password123456/setup-selenium-with-chrome-driver-on-ubuntu_debian
https://developer.chrome.com/docs/chromedriver/get-started

https://github.com/bonigarcia/rust-examples

https://www.zenrows.com/blog/rust-selenium
https://dev.to/stevepryde/using-selenium-with-rust-aca
https://www.twilio.com/en-us/blog/developers/community/web-scraping-rust-selenium
https://iproyal.com/blog/rust-web-scraping-with-selenium/
https://docs.rs/thirtyfour/latest/thirtyfour/

sudo apt install python3-pip
pip3 install selenium webdriver-manager
+chrome
sudo apt install chromium-chromedriver
.\chromedriver
//ChromeDriver was started successfully on port 36765
cargo run

// visiting google.com,
// searching for the keyword "rust",
// and scraping its contents
*/

use thirtyfour::prelude::*;
//use thirtyfour::Key;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    println!("    caps: {:#?}", caps);
    let driver = WebDriver::new("http://localhost:36765", caps).await?;
    //let driver = WebDriver::new("http://localhost:9515", "chrome").await?;

    //driver.goto("https://www.google.com").await?;
    driver.goto("https://alvmiller.github.io/web_page_example/").await?;

    let page_id_header = driver.find(By::Id("myHeader")).await?;
    println!("    page_id_header: {:#?}", page_id_header);
/*
    let search_box = driver.find(By::Id("q")).await?;
    search_box.send_keys("rust").await?;
    let message_key = format!("{:#?}", Key::Enter);
    println!("    message_key: {}", message_key);
    search_box.send_keys(message_key).await?;
*/
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let page_source = driver.source().await?;
    println!("{}", page_source);
    driver.quit().await?;

    Ok(())
}
