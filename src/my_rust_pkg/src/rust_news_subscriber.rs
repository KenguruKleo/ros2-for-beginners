use anyhow::Result;
use example_interfaces::msg::String;
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};

fn main() -> Result<()> {
    let context = Context::default_from_env()?;
    let mut _executor = context.create_basic_executor();
    let node = _executor.create_node("rust_news_subscriber")?;
    rclrs::log_info!(node.logger(), "Created rust_news_subscriber node!");

    let log_node = node.clone();
    let _subscriber = node.create_subscription("robot_news", move |msg: String| {
        rclrs::log_info!(log_node.logger(), "Received: {}", msg.data);
    })?;

    _executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}