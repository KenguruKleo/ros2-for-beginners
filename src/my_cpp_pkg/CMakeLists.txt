cmake_minimum_required(VERSION 3.8)
project(my_cpp_pkg)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(example_interfaces REQUIRED)
find_package(my_robot_interfaces REQUIRED)
find_package(rclcpp REQUIRED)

add_executable(cpp_node src/my_first_node.cpp)
ament_target_dependencies(cpp_node rclcpp)

add_executable(add_two_ints_client src/add_two_ints_client.cpp)
ament_target_dependencies(add_two_ints_client example_interfaces rclcpp)

add_executable(add_two_ints_serv src/add_two_ints_serv.cpp)
ament_target_dependencies(add_two_ints_serv example_interfaces rclcpp)

add_executable(hardware_status_publisher src/hardware_status_publisher.cpp)
ament_target_dependencies(hardware_status_publisher my_robot_interfaces rclcpp)

install(TARGETS
  add_two_ints_client
  add_two_ints_serv
  cpp_node
  hardware_status_publisher
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
