#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from my_robot_interfaces.msg import HardwareStatus

class MyNode(Node):
    def __init__(self):
        node_name = 'hardware_status_publisher'
        super().__init__(node_name)
        self.get_logger().info(f'{node_name} started!')

        self.publisher_ = self.create_publisher(HardwareStatus, 'hardware_status', 10)
        timer_period = 1.0  # seconds
        self.timer = self.create_timer(timer_period, self.timer_callback)

    def timer_callback(self):
        msg = HardwareStatus()
        msg.temperature = 50.0  # Example temperature in Celsius
        msg.are_motors_ready = True  # Example motor status
        msg.debug_message = 'All systems nominal'  # Example debug message
        self.publisher_.publish(msg)
        self.get_logger().info('Publishing hardware status: Temperature: %.2f°C, Motors Ready: %s' % (msg.temperature, msg.are_motors_ready))

def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
