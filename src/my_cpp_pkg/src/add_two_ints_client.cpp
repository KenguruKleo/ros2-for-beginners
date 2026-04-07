#include <chrono>
#include <functional>
#include <memory>

#include "example_interfaces/srv/add_two_ints.hpp"
#include "rclcpp/rclcpp.hpp"

using namespace std::chrono_literals;

class AddTwoIntsClientNode : public rclcpp::Node
{
public:
  AddTwoIntsClientNode()
  : Node("add_two_ints_client")
  {
    client_ = this->create_client<example_interfaces::srv::AddTwoInts>("add_two_ints");
  }

  void add_two_ints(int64_t a, int64_t b)
  {
    while (!client_->wait_for_service(1s)) {
      if (!rclcpp::ok()) {
        RCLCPP_ERROR(this->get_logger(), "Interrupted while waiting for the service.");
        return;
      }

      RCLCPP_INFO(
        this->get_logger(),
        "Service \"add_two_ints\" not available, waiting again...");
    }

    auto request = std::make_shared<example_interfaces::srv::AddTwoInts::Request>();
    request->a = a;
    request->b = b;

    client_->async_send_request(
      request,
      std::bind(&AddTwoIntsClientNode::callback_add_two_ints, this, std::placeholders::_1));
  }

private:
  using AddTwoInts = example_interfaces::srv::AddTwoInts;
  using SharedFuture = rclcpp::Client<AddTwoInts>::SharedFuture;

  rclcpp::Client<AddTwoInts>::SharedPtr client_;

  void callback_add_two_ints(SharedFuture future)
  {
    try {
      auto response = future.get();
      RCLCPP_INFO(this->get_logger(), "Result of add_two_ints: %ld!", response->sum);
    } catch (const std::exception & error) {
      RCLCPP_ERROR(this->get_logger(), "Service call failed: %s", error.what());
    }
  }
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  auto node = std::make_shared<AddTwoIntsClientNode>();
  node->add_two_ints(3, 5);
  node->add_two_ints(31, 3);
  node->add_two_ints(1, 8);

  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}
