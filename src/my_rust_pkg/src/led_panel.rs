use anyhow::Result;
use my_robot_interfaces::msg::LedStateArray;
use my_robot_interfaces::srv::{SetLed, SetLed_Request, SetLed_Response};
use rclrs::{Context, CreateBasicExecutor, RclrsErrorFilter, SpinOptions};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn publish_led_status(
    publisher: &rclrs::Publisher<LedStateArray>,
    led_states: &[i64],
) -> Result<()> {
    let msg = LedStateArray {
        led_states: led_states.to_vec(),
    };
    publisher.publish(&msg)?;
    println!("Publishing LED status: {:?}", msg.led_states);
    Ok(())
}

fn main() -> Result<()> {
    let context: Context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("led_panel")?;
    println!("led_panel started!");

    let timer_period = node
        .declare_parameter("timer_period")
        .default(5.0)
        .mandatory()?;

    let led_states = Arc::new(Mutex::new(vec![0_i64, 0, 0]));
    let publisher = node.create_publisher::<LedStateArray>("led_status")?;

    let service_led_states = Arc::clone(&led_states);
    let service_publisher = Arc::clone(&publisher);
    let _service = node.create_service::<SetLed, _>("set_led", move |request: SetLed_Request| {
        let mut states = service_led_states.lock().unwrap();
        let success = if request.led_number >= 0
            && (request.led_number as usize) < states.len()
        {
            states[request.led_number as usize] = request.led_state;
            true
        } else {
            false
        };

        if let Err(error) = publish_led_status(&service_publisher, &states) {
            eprintln!("Failed to publish LED status after service call: {error:?}");
        }

        SetLed_Response { success }
    })?;

    let timer_led_states = Arc::clone(&led_states);
    let timer_publisher = Arc::clone(&publisher);
    let _timer = node.create_timer_repeating(
        Duration::from_secs_f64(timer_period.get()),
        move || {
            let states = timer_led_states.lock().unwrap();
            if let Err(error) = publish_led_status(&timer_publisher, &states) {
                eprintln!("Failed to publish LED status on timer tick: {error:?}");
            }
        },
    )?;

    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
