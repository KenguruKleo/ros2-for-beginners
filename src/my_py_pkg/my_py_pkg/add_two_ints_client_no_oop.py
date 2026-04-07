#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts

class MyNode(Node):
    def __init__(self):
        super().__init__('add_two_ints_client_no_oop')

def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    client = node.create_client(AddTwoInts, 'add_two_ints')
    while not client.wait_for_service(timeout_sec=1.0):
        node.get_logger().info('Service "add_two_ints" not available, waiting again...')

    feature = client.call_async(AddTwoInts.Request(a=3, b=5))
    rclpy.spin_until_future_complete(node, feature)
    result: AddTwoInts.Response | None = feature.result()
    if result is not None:
        node.get_logger().info(f'Result of add_two_ints: {result.sum}')

    # rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
