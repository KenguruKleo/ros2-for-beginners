use anyhow::Result;
use example_interfaces::srv::{AddTwoInts, AddTwoInts_Request, AddTwoInts_Response};
use rclrs::{Client, Context, CreateBasicExecutor, Node, RclrsErrorFilter, SpinOptions, Timer};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

const NODE_NAME: &str = "add_two_ints_client";
const SERVICE_NAME: &str = "/add_two_ints";

#[derive(Clone, Copy)]
struct Operands {
    a: i64,
    b: i64,
}

struct RequestLoopState {
    next_a: Mutex<i64>,
    request_in_flight: AtomicBool,
}

impl RequestLoopState {
    fn new() -> Self {
        Self {
            next_a: Mutex::new(1),
            request_in_flight: AtomicBool::new(false),
        }
    }

    fn try_start_request(&self) -> bool {
        self.request_in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn next_operands(&self) -> Operands {
        let mut next_a = self.next_a.lock().unwrap();
        let operands = Operands {
            a: *next_a,
            b: *next_a + 10,
        };
        *next_a += 1;
        operands
    }

    fn finish_request(&self) {
        self.request_in_flight.store(false, Ordering::Release);
    }
}

struct MainNode {
    _node: Node,
    _client: Client<AddTwoInts>,
    _request_timer: Timer,
    _heartbeat_timer: Timer,
}

impl MainNode {
    fn new(executor: &rclrs::Executor) -> Result<Self> {
        let node = executor.create_node(NODE_NAME)?;
        let client = node.create_client::<AddTwoInts>(SERVICE_NAME)?;
        let request_state = Arc::new(RequestLoopState::new());

        let request_timer = Self::create_request_timer(&node, &client, request_state)?;
        let heartbeat_timer = Self::create_heartbeat_timer(&node)?;

        Ok(Self {
            _node: node,
            _client: client,
            _request_timer: request_timer,
            _heartbeat_timer: heartbeat_timer,
        })
    }

    fn create_heartbeat_timer(node: &Node) -> Result<Timer> {
        let heartbeat_node = node.clone();

        Ok(node.create_timer_repeating(Duration::from_secs(1), move || {
            rclrs::log_info!(
                heartbeat_node.logger(),
                "Client node is alive and can do other work while waiting for responses."
            );
        })?)
    }

    fn create_request_timer(
        node: &Node,
        client: &Client<AddTwoInts>,
        request_state: Arc<RequestLoopState>,
    ) -> Result<Timer> {
        let request_node = node.clone();
        let request_client = client.clone();
        let request_state_for_timer = Arc::clone(&request_state);

        Ok(node.create_timer_repeating(Duration::from_secs(2), move || {
            let service_is_ready = match request_client.service_is_ready() {
                Ok(is_ready) => is_ready,
                Err(err) => {
                    rclrs::log_error!(
                        request_node.logger(),
                        "Failed to check service availability: {err:?}"
                    );
                    return;
                }
            };

            if !service_is_ready {
                rclrs::log_info!(
                    request_node.logger(),
                    "Service \"{}\" not available yet; will try again on the next tick.",
                    SERVICE_NAME
                );
                return;
            }

            if !request_state_for_timer.try_start_request() {
                rclrs::log_warn!(
                    request_node.logger(),
                    "Previous request is still in flight; skipping this tick."
                );
                return;
            }

            let operands = request_state_for_timer.next_operands();
            Self::send_add_two_ints_request(
                request_node.clone(),
                request_client.clone(),
                Arc::clone(&request_state_for_timer),
                operands,
            );
        })?)
    }

    fn send_add_two_ints_request(
        node: Node,
        client: Client<AddTwoInts>,
        request_state: Arc<RequestLoopState>,
        operands: Operands,
    ) {
        let request = AddTwoInts_Request {
            a: operands.a,
            b: operands.b,
        };
        let response_node = node.clone();
        let response_state = Arc::clone(&request_state);

        rclrs::log_info!(node.logger(), "Sending request: a={}, b={}", request.a, request.b);

        if let Err(err) = client.call_then(&request, move |response: AddTwoInts_Response| {
            rclrs::log_info!(
                response_node.logger(),
                "Result of add_two_ints({}, {}): {}",
                operands.a,
                operands.b,
                response.sum
            );
            response_state.finish_request();
        }) {
            request_state.finish_request();
            rclrs::log_error!(node.logger(), "Failed to send request: {err:?}");
        }
    }
}

fn main() -> Result<()> {
    let context: Context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let _app = MainNode::new(&executor)?;
    println!("{} started!", NODE_NAME);

    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
