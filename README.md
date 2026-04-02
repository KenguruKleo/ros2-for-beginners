## install ROS2

https://docs.ros.org/en/humble/Installation.html

## install Rust

https://github.com/ros2-rust/ros2_rust

## Build and run packages

`colcon` is used to build the workspace. `ros2` is used to run nodes and inspect ROS 2 packages.

Typical workflow:

```bash
cd /home/konst/workspace/ros2_ws
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
```

How names are mapped:

- `package_name` comes from `package.xml`
- Python executable names come from `setup.py` `console_scripts`
- C++ executable names come from `add_executable(...)` in `CMakeLists.txt`
- Rust executable names come from `[[bin]] name` in `Cargo.toml`

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
