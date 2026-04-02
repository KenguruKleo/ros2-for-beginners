use anyhow::Result;
use example_interfaces::msg::String;
use rclrs::{Context, CreateBasicExecutor};
use std::time::Duration;

fn main() -> Result<()> {
    let context = Context::default_from_env()?;
    let executor = context.create_basic_executor();
    let node = executor.create_node("rust_news_station")?;

    let publisher = node.create_publisher::<String>("robot_news")?;

    let mut msg = String::default();

    loop {
        msg.data = "Hello from Rust ROS2!".to_string();
        publisher.publish(&msg)?;
        println!("Published: {}", msg.data);

        std::thread::sleep(Duration::from_secs(1));
    }
}
