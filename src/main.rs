use std::fs;
use headless_chrome::{
    browser::{Browser, LaunchOptionsBuilder},
    protocol::page::ScreenshotFormat};
use failure::err_msg;

// One can replace hardcoded values with environment variables or command line arguments.
const LOGIN: &str = "";
const PASSWORD: &str = "";

fn main() -> Result<(), failure::Error> {

    let mut lo_builder = LaunchOptionsBuilder::default();
    lo_builder.window_size(Some((1280, 1024)));
    let launch_options = lo_builder.build().map_err(err_msg)?;

    let browser = Browser::new(launch_options)?;

    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(std::time::Duration::from_millis(5000));

    println!("[1/8] Navigating to VK");
    tab.navigate_to("https://vk.com/")?;
    
    println!("[2/8] Navigating to sign in page");
    tab.wait_for_element("button.VkIdForm__signInButton")?.click()?;
    tab.wait_until_navigated()?;
    
    println!("[3/8] Attempting to enter login");
    tab.find_element("input.vkc__TextField__input[name=login]")?.click()?;
    tab.type_str(LOGIN)?;

    println!("[4/8] Attempting to submit login");
    tab.find_element("button[type=submit]")?.click()?;

    println!("[5/8] Attempting to enter password");
    tab.wait_for_element("input.vkc__TextField__input[name=password]")?.click()?;
    tab.type_str(PASSWORD)?;

    println!("[6/8] Attempting to submit password");
    tab.find_element("button[type=submit]")?.click()?;

    println!("[7/8] Waiting for the main page to load");
    tab.wait_for_element("a.TopHomeLink")?;

    println!("[8/8] Screenshotting the main page");
    let jpeg_data = tab.capture_screenshot(
        ScreenshotFormat::JPEG(Some(75)),
        None,
        true)?;
    fs::write("screenshot.jpeg", jpeg_data)?;
    Ok(())
}
