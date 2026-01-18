use anyhow::Result;
use yagura::{app::App, process::Pid};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();

    if app.should_quit() {
        println!("App is set to quit.");
    } else {
        println!("App is running.");
    }

    let pid = Pid(1234);
    println!("Hello, world! {}", pid);

    app.quit();

    if app.should_quit() {
        println!("App is set to quit.");
    } else {
        println!("App is running.");
    }

    Ok(())
}
