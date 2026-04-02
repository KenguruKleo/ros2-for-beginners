#!/usr/bin/env python3
from my_py_pkg.template import MyNode
import rclpy
from rclpy.node import Node
from example_interfaces.msg import String

class RoborNewsStationNode(Node):
    def __init__(self):
        super().__init__('robot_news_station')
        self.publisher_ = self.create_publisher(String, 'robot_news', 10)
        self.timer_ = self.create_timer(1.0, self.publish_news)
        self.get_logger().info('Robot News Station Node has been started.')

    def publish_news(self):
        msg = String()
        msg.data = "Breaking news: ROS 2 is awesome!"
        self.publisher_.publish(msg)
        self.get_logger().info(f'Published news: "{msg.data}"')

def main(args=None):
    rclpy.init(args=args)
    node = RoborNewsStationNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
