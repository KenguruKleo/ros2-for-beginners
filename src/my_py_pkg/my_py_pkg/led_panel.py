#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from my_robot_interfaces.msg import LedStateArray
from my_robot_interfaces.srv import SetLed

class MyNode(Node):
    def __init__(self):
        node_name = 'led_panel'
        super().__init__(node_name)
        self.get_logger().info(f'{node_name} started!')

        self.declare_parameter('timer_period', 5.0)
        timer_period: float = self.get_parameter(
            'timer_period').get_parameter_value().double_value

        self.led_states = [0, 0, 0]

        # Service for setting LED state
        self.set_led_service_ = self.create_service(SetLed, 'set_led', self.callback_set_led)

        # Publisher for LED status
        self.led_status_publisher_ = self.create_publisher(LedStateArray, 'led_status', 10)
        self.timer = self.create_timer(timer_period, self.timer_callback)

    def timer_callback(self):
        msg = LedStateArray()
        msg.led_states = self.led_states
        self.led_status_publisher_.publish(msg)
        self.get_logger().info('Publishing LED status: %s' % str(msg.led_states))

    def callback_set_led(self, request: SetLed.Request, response: SetLed.Response):
        if 0 <= request.led_number < len(self.led_states):
            self.led_states[request.led_number] = request.led_state
            response.success = True
        else:
            response.success = False

        # Publish the updated LED states after setting the LED
        msg = LedStateArray()
        msg.led_states = self.led_states
        self.led_status_publisher_.publish(msg)
        
        return response

def main(args=None):
    rclpy.init(args=args)
    node = MyNode()

    rclpy.spin(node)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
