use anyhow::Result;
use yagura::process::Pid;

#[tokio::main]
async fn main() -> Result<()> {
    let pid = Pid(1234);
    println!("Hello, world! {}", pid);
    Ok(())
}
