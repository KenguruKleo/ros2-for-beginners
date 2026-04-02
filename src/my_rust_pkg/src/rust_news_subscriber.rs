use anyhow::Result;
use example_interfaces::msg::String;
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};

fn main() -> Result<()> {
    let context = Context::default_from_env()?;
    let mut _executor = context.create_basic_executor();
    let node = _executor.create_node("rust_news_subscriber")?;
     println!("Created rust_news_subscriber node!");

    let _subscriber = node.create_subscription("robot_news", |msg: String| {
        println!("Received: {}", msg.data);
    })?;

    _executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}