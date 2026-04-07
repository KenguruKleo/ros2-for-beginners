#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts

class MyNode(Node):
    def __init__(self):
        super().__init__('add_two_ints_serv')
        self.srv = self.create_service(AddTwoInts, 'add_two_ints', self.add_two_ints_callback)
        self.get_logger().info('Service "add_two_ints" is ready to receive requests.')

    def add_two_ints_callback(self, request: AddTwoInts.Request, response: AddTwoInts.Response) -> AddTwoInts.Response:
        response.sum = request.a + request.b
        self.get_logger().info(
            f'Received request: a={request.a}, b={request.b}. Responding with sum={response.sum}.'
        )
        return response
    
def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
