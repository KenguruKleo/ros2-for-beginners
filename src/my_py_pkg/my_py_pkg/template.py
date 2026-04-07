#!/usr/bin/env python3
import rclpy
from rclpy.node import Node

class MyNode(Node):
    def __init__(self):
        node_name = 'node_name' #replace 'node_name' with the name of your node
        super().__init__(node_name)
        self.get_logger().info(f'{node_name} started!')

def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
