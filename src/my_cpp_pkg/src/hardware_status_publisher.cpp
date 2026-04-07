#include <chrono>
#include <memory>

#include "my_robot_interfaces/msg/hardware_status.hpp"
#include "rclcpp/rclcpp.hpp"

using namespace std::chrono_literals;

class HardwareStatusPublisher : public rclcpp::Node
{
public:
  HardwareStatusPublisher()
  : Node("hardware_status_publisher")
  {
    RCLCPP_INFO(this->get_logger(), "hardware_status_publisher started!");

    publisher_ = this->create_publisher<my_robot_interfaces::msg::HardwareStatus>(
      "hardware_status", 10);
    timer_ = this->create_wall_timer(
      1s, std::bind(&HardwareStatusPublisher::timer_callback, this));
  }

private:
  rclcpp::Publisher<my_robot_interfaces::msg::HardwareStatus>::SharedPtr publisher_;
  rclcpp::TimerBase::SharedPtr timer_;

  void timer_callback()
  {
    auto msg = my_robot_interfaces::msg::HardwareStatus();
    msg.temperature = 50.0;
    msg.are_motors_ready = true;
    msg.debug_message = "All systems nominal";

    publisher_->publish(msg);
    RCLCPP_INFO(
      this->get_logger(),
      "Publishing hardware status: Temperature: %.2f C, Motors Ready: %s",
      msg.temperature,
      msg.are_motors_ready ? "True" : "False");
  }
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  auto node = std::make_shared<HardwareStatusPublisher>();

  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}
