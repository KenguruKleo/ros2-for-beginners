#!/usr/bin/env python3
from my_py_pkg.my_py_pkg.template import MyNode
import rclpy
from rclpy.node import Node
from example_interfaces.msg import String

class RoborNewsStationNode(Node):
    def __init__(self):
        super().__init__('robot_news_station')
        self.publisher_ = self.create_publisher(String, 'robot_news', 10)

def main(args=None):
    rclpy.init(args=args)
    node = RoborNewsStationNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
