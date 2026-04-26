use resonate::prelude::*;
use std::time::Duration;

/// A workflow that sleeps durably for the requested number of seconds.
///
/// `ctx.sleep(...)` creates a server-backed timer promise and suspends the
/// workflow until that promise resolves. The suspension survives process
/// crashes — if the worker dies mid-sleep, another worker (or the same one
/// after restart) resumes the workflow when the timer fires.
#[resonate::function]
async fn sleeping_workflow(ctx: &Context, secs: u64) -> Result<String> {
    println!("Sleeping for {secs} seconds...");

    ctx.sleep(Duration::from_secs(secs)).await?;

    Ok(format!("Slept for {secs} seconds"))
}

#[tokio::main]
async fn main() {
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("workers".into()),
        ..Default::default()
    });

    resonate.register(sleeping_workflow).unwrap();

    println!("Worker started. Waiting for invocations...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}
