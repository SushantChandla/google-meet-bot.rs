use dotenv::dotenv;
use futures::future::join_all;
use std::{env, time::Duration};
use thirtyfour::{error::WebDriverError, prelude::*, support::sleep};
use tokio::{self};

/// How to use
/// Create a .env file in the root directory
/// Add the email id and password
///
/// ❯ cat .env
/// email = email@gmail.com
/// password = password
///
/// Run the project:
/// Download the chrome driver from https://chromedriver.chromium.org/
/// `chromedriver --port=4444`  
/// `cargo run`

const TOTAL_WINDOWS: u64 = 2;
const MEET_LINK: &str = "https://meet.google.com/jce-syxe-nuo";
const WAIT_SECONDS: u64 = 4;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut tasks = vec![];
    for _i in 0..TOTAL_WINDOWS {
        tasks.push(join_google_meet());
    }
    for i in join_all(tasks).await {
        i.unwrap()
    }
}

async fn join_google_meet() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("use-fake-ui-for-media-stream")?;

    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    driver.get("https://meet.google.com").await?;
    if driver.title().await? != "Google Meet".to_string() {
        google_login(&driver).await?;
    }
    wait().await;

    driver.get(MEET_LINK).await?;

    wait().await;

    let join_meet = &driver.find_elements(By::ClassName("uArJ5e")).await?[1];
    join_meet.click().await
}

async fn google_login(driver: &WebDriver) -> WebDriverResult<()> {
    driver.get("https://accounts.google.com/signin/v2").await?;
    let mut elem_form = driver.find_element(By::Css("input[type='email']")).await?;

    let email = match env::var("email") {
        Ok(a) => a,
        Err(_) => {
            return Err(WebDriverError::FatalError(
                "Email Address not found".to_string(),
            ))
        }
    };

    elem_form.send_keys(email).await?;
    elem_form = driver.find_element(By::Id("identifierNext")).await?;
    elem_form.click().await?;
    wait().await;

    let password = match env::var("password") {
        Ok(a) => a,
        Err(_) => return Err(WebDriverError::FatalError("Password not found".to_string())),
    };

    elem_form = driver
        .find_element(By::Css("input[type='password']"))
        .await?;

    elem_form.send_keys(password).await?;
    elem_form = driver.find_element(By::Id("passwordNext")).await?;
    elem_form.click().await?;

    Ok(())
}

async fn wait() {
    sleep(Duration::new(WAIT_SECONDS, 0)).await;
}
