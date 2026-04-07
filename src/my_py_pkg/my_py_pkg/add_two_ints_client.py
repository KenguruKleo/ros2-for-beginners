#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts

class MyNode(Node):
    def __init__(self):
        super().__init__('add_two_ints_client')
        self.client = self.create_client(AddTwoInts, 'add_two_ints')
    
    def add_two_ints(self, a: int, b: int):
        while not self.client.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('Service "add_two_ints" not available, waiting again...')

        request = AddTwoInts.Request(a=a, b=b)
        future = self.client.call_async(request)
        future.add_done_callback(self.callback_add_two_ints)

    def callback_add_two_ints(self, future):
        try:
            response = future.result()
            self.get_logger().info(f'Result of add_two_ints: {response.sum}!')
        except Exception as e:
            self.get_logger().error(f'Service call failed: {e}')


def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    node.add_two_ints(3, 5)
    node.add_two_ints(31, 3)
    node.add_two_ints(1, 8)
    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
