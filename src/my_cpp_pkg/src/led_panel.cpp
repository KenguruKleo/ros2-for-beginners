#include <chrono>
#include <memory>
#include <vector>

#include "my_robot_interfaces/msg/led_state_array.hpp"
#include "my_robot_interfaces/srv/set_led.hpp"
#include "rclcpp/rclcpp.hpp"

using namespace std::chrono_literals;
using SetLed = my_robot_interfaces::srv::SetLed;
using LedStateArray = my_robot_interfaces::msg::LedStateArray;

class LedPanelNode : public rclcpp::Node
{
public:
  LedPanelNode()
  : Node("led_panel"), led_states_{0, 0, 0}
  {
    RCLCPP_INFO(this->get_logger(), "led_panel started!");

    const auto timer_period = this->declare_parameter<double>("timer_period", 5.0);

    set_led_service_ = this->create_service<SetLed>(
      "set_led",
      std::bind(
        &LedPanelNode::callback_set_led,
        this,
        std::placeholders::_1,
        std::placeholders::_2));

    led_status_publisher_ = this->create_publisher<LedStateArray>("led_status", 10);
    timer_ = this->create_wall_timer(
      std::chrono::duration<double>(timer_period),
      std::bind(&LedPanelNode::timer_callback, this));
  }

private:
  std::vector<int64_t> led_states_;
  rclcpp::Service<SetLed>::SharedPtr set_led_service_;
  rclcpp::Publisher<LedStateArray>::SharedPtr led_status_publisher_;
  rclcpp::TimerBase::SharedPtr timer_;

  void publish_led_status()
  {
    auto msg = LedStateArray();
    msg.led_states = led_states_;
    led_status_publisher_->publish(msg);
    RCLCPP_INFO(this->get_logger(), "Publishing LED status: [%ld, %ld, %ld]",
      led_states_[0], led_states_[1], led_states_[2]);
  }

  void timer_callback()
  {
    publish_led_status();
  }

  void callback_set_led(
    const SetLed::Request::SharedPtr request,
    SetLed::Response::SharedPtr response)
  {
    if (request->led_number >= 0 &&
      static_cast<size_t>(request->led_number) < led_states_.size())
    {
      led_states_[request->led_number] = request->led_state;
      response->success = true;
    } else {
      response->success = false;
    }

    publish_led_status();
  }
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  auto node = std::make_shared<LedPanelNode>();

  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}
