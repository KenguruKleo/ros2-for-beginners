use anyhow::Result;
use example_interfaces::srv::{AddTwoInts, AddTwoInts_Request, AddTwoInts_Response};
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};

/// Creates a ROS 2 context and node, prints a hello message,
/// then spins until shutdown.
fn main() -> Result<()> {
    let context: Context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let _node_name = "add_two_ints_serv";
    let _node = executor.create_node(_node_name)?;
    println!("{} started!", _node_name);

    // Keep the service handle alive for as long as the node is spinning.
    let _service = _node.create_service::<AddTwoInts, _>("add_two_ints", |request: AddTwoInts_Request| {
        let response = AddTwoInts_Response {
            sum: request.a + request.b,
        };
        println!(
            "Received request: a={}, b={}. Responding with sum={}.",
            request.a, request.b, response.sum
        );
        response
    })?;

    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
