#include <memory>

#include "example_interfaces/srv/add_two_ints.hpp"
#include "rclcpp/rclcpp.hpp"

class AddTwoIntsServiceNode : public rclcpp::Node
{
public:
  AddTwoIntsServiceNode()
  : Node("add_two_ints_serv")
  {
    service_ = this->create_service<example_interfaces::srv::AddTwoInts>(
      "add_two_ints",
      std::bind(
        &AddTwoIntsServiceNode::handle_add_two_ints,
        this,
        std::placeholders::_1,
        std::placeholders::_2));

    RCLCPP_INFO(
      this->get_logger(),
      "Service \"add_two_ints\" is ready to receive requests.");
  }

private:
  rclcpp::Service<example_interfaces::srv::AddTwoInts>::SharedPtr service_;

  void handle_add_two_ints(
    const std::shared_ptr<example_interfaces::srv::AddTwoInts::Request> request,
    std::shared_ptr<example_interfaces::srv::AddTwoInts::Response> response)
  {
    response->sum = request->a + request->b;

    RCLCPP_INFO(
      this->get_logger(),
      "Received request: a=%ld, b=%ld. Responding with sum=%ld.",
      request->a,
      request->b,
      response->sum);
  }
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  auto node = std::make_shared<AddTwoIntsServiceNode>();
  rclcpp::spin(node);

  rclcpp::shutdown();
  return 0;
}
