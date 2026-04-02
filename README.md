## install ROS2

https://docs.ros.org/en/humble/Installation.html

## install Rust

https://github.com/ros2-rust/ros2_rust

## Build and run packages

`colcon` is used to build the workspace. `ros2` is used to run nodes and inspect ROS 2 packages.

Typical workflow:

```bash
cd /home/konst/ros2_ws
source /opt/ros/humble/setup.bash
colcon build
source install/setup.bash
```

Then run a node with:

```bash
ros2 run <package_name> <executable_name>
```

Examples from this workspace:

```bash
ros2 run my_py_pkg py_node
ros2 run my_cpp_pkg cpp_node
ros2 run my_rust_pkg my_rust_pkg
ros2 run my_rust_pkg rust_news_station
ros2 run my_rust_pkg rust_news_subscriber
```

How names are mapped:

- `package_name` comes from `package.xml`
- Python executable names come from `setup.py` `console_scripts`
- C++ executable names come from `add_executable(...)` in `CMakeLists.txt`
- Rust executable names come from `[[bin]] name` in `Cargo.toml`
- ROS 2 node names come from the source code and can be different from the executable name

Useful commands:

```bash
colcon build --packages-select my_py_pkg
ros2 pkg list
ros2 pkg executables my_py_pkg
```

## Environment setup

Use both setup files, but for different purposes:

- `source /opt/ros/humble/setup.bash` loads the base ROS 2 installation
- `source install/setup.bash` overlays your local workspace so ROS 2 can see packages built in `src/`

If you open a new terminal, run the setup commands again before using `ros2 run`.

## Show interfaces
```
ros2 interface show example_interfaces/msg/String 
```

## Node commands

Examples of executable names and the ROS 2 node names they start in this workspace:

- `ros2 run my_rust_pkg my_rust_pkg` starts `/my_rust_pkg_node`
- `ros2 run my_rust_pkg rust_news_station` starts `/rust_news_station`
- `ros2 run my_rust_pkg rust_news_subscriber` starts `/rust_news_subscriber`
- `ros2 run my_py_pkg py_node` starts `/py_test`
- `ros2 run my_py_pkg robot_news_station` starts `/robot_news_station`
- `ros2 run my_cpp_pkg cpp_node` starts `/my_first_node`

List all running nodes:

```bash
ros2 node list
```

If the Rust publisher and subscriber are running, you should see node names like:

```text
/rust_news_station
/rust_news_subscriber
```

Show details about a node:

```bash
ros2 node info /rust_news_station
ros2 node info /rust_news_subscriber
```

`ros2 node info` shows publishers, subscribers, services, and actions for that node.

Useful commands for nodes:

```bash
ros2 node list
ros2 node info /my_rust_pkg_node
ros2 node info /py_test
ros2 node info /my_first_node
ros2 param list /rust_news_station
ros2 param get /rust_news_station use_sim_time
ros2 topic list -t
```

## Topic commands with the Rust example

This workspace contains two Rust nodes that use the `robot_news` topic with the `example_interfaces/msg/String` message type:

- `ros2 run my_rust_pkg rust_news_station` publishes messages on `robot_news`
- `ros2 run my_rust_pkg rust_news_subscriber` subscribes to `robot_news`

Open a terminal for each step and source ROS 2 plus the workspace in every terminal:

```bash
cd /home/konst/ros2_ws
source /opt/ros/humble/setup.bash
source install/setup.bash
```

Run the Rust publisher:

```bash
ros2 run my_rust_pkg rust_news_station
```

Run the Rust subscriber:

```bash
ros2 run my_rust_pkg rust_news_subscriber
```

List topics:

```bash
ros2 topic list
```

You should see `/robot_news` in the output when the Rust publisher or subscriber is running.

Show information about the topic:

```bash
ros2 topic info /robot_news
```

Echo messages from the topic:

```bash
ros2 topic echo /robot_news
```

If `rust_news_station` is running, you should see messages like:

```text
data: Hello from Rust ROS2!
```

Publish a test message to the topic from the CLI:

```bash
ros2 topic pub --once /robot_news example_interfaces/msg/String "{data: 'Hello from ROS 2 CLI'}"
```

If `rust_news_subscriber` is running, it will print the received message.

Publish messages continuously:

```bash
ros2 topic pub /robot_news example_interfaces/msg/String "{data: 'Breaking news from terminal'}"
```
