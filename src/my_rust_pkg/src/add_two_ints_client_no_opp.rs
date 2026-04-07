use anyhow::Result;
use example_interfaces::srv::{AddTwoInts, AddTwoInts_Request, AddTwoInts_Response};
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    const SERVICE_NAME: &str = "/add_two_ints";

    let context: Context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node_name = "add_two_ints_client";
    let node = executor.create_node(node_name)?;
    println!("{} started!", node_name);

    let client = node.create_client::<AddTwoInts>(SERVICE_NAME)?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while !client.service_is_ready()? {
        if Instant::now() >= deadline {
            anyhow::bail!("Service {SERVICE_NAME} did not become available within 5 seconds");
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    let request = AddTwoInts_Request { a: 3, b: 4 };
    println!("Sending request: a={}, b={}", request.a, request.b);

    let promise = client.call_then(&request, |response: AddTwoInts_Response| {
        println!("Received response: sum={}", response.sum);
    })?;

    executor
        .spin(
            SpinOptions::default()
                .until_promise_resolved(promise)
                .timeout(Duration::from_secs(5)),
        )
        .first_error()?;

    Ok(())
}
