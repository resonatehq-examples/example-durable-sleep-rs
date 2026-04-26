use resonate::prelude::*;

/// Invoke the sleeping workflow on a worker and await the result.
///
/// Resonate deduplicates by promise ID: invoking with a workflow ID that
/// already has a PENDING execution reconnects to it; one that has RESOLVED
/// returns the cached result.
#[tokio::main]
async fn main() {
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("client".into()),
        ..Default::default()
    });

    let id = "sleep-workflow-1";
    let secs: u64 = 5;

    let result: String = resonate
        .rpc(id, "sleeping_workflow", secs)
        .target("poll://any@workers")
        .await
        .expect("rpc to worker failed");

    println!("{result}");
}
