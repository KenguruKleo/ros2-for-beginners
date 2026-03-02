#!/usr/bin/env python3
import rclpy
from rclpy.node import Node

class MyNode(Node):
    def __init__(self):
        super().__init__('py_test')
        self._counter = 0

        self.get_logger().info('Hello World')

        self.create_timer(1.0, self.timer_callback)

    def timer_callback(self):
        self._counter += 1
        self.get_logger().info(f'Timer callback called {self._counter} times')

def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
