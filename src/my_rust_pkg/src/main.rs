use anyhow::Result;
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};

fn main() -> Result<()> {
    let context: Context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let _node_name = "my_rust_pkg_node";  // replace "my_rust_pkg_node" with the name of your node
    let _node = executor.create_node(_node_name)?;
    println!("{} started!", _node_name);

    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
