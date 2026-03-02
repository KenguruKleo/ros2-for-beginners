#include <memory>

#include "rclcpp/rclcpp.hpp"

class MyNode : public rclcpp::Node
{
public:
  MyNode() : Node("my_first_node"), counter_(0)
  {
    RCLCPP_INFO(this->get_logger(), "Hello, ROS 2 Cpp!");
    auto timer_callback = std::bind(&MyNode::timer_callback, this);
    this->timer_ = this->create_wall_timer(std::chrono::seconds(1), timer_callback);
  }

private:
  int counter_;
  rclcpp::TimerBase::SharedPtr timer_;

  void timer_callback()
  {
    this->counter_++;
    RCLCPP_INFO(this->get_logger(), "Timer callback called %d times", counter_);
  }
};

int main(int argc, char **argv)
{
  rclcpp::init(argc, argv);

  auto node = std::make_shared<MyNode>();

  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}
