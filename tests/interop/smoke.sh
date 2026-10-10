#!/usr/bin/env bash
# Phase 0 smoke test: proves the interop harness can run stock ROS 2 at all.
#
# Runs inside the image from tests/interop/Dockerfile. Set RMW_IMPLEMENTATION to choose
# the middleware (rmw_fastrtps_cpp or rmw_cyclonedds_cpp). Prints one PASS or FAIL line.
set -eo pipefail

# ROS setup scripts read unset variables, so `set -u` stays off while sourcing.
source /opt/ros/jazzy/setup.bash
set -u

fail() { echo "FAIL: $*"; exit 1; }

rmw="${RMW_IMPLEMENTATION:-rmw_fastrtps_cpp}"
echo "middleware: ${rmw}"

# Start a stock C++ talker so there is a real node and topic to discover.
ros2 run demo_nodes_cpp talker > /tmp/talker.log 2>&1 &
talker_pid=$!
trap 'kill "${talker_pid}" 2>/dev/null || true' EXIT

# Discovery takes a moment; poll instead of sleeping a fixed time.
for _ in $(seq 1 20); do
  if timeout 10 ros2 topic list --no-daemon 2>/dev/null | grep -qx /chatter; then
    message="$(timeout 10 ros2 topic echo --once /chatter std_msgs/msg/String 2>/dev/null || true)"
    [[ "${message}" == *"Hello World"* ]] || fail "found /chatter but could not read a message from it"
    echo "PASS: ${rmw}: discovered /chatter and received: $(head -n1 <<< "${message}")"
    exit 0
  fi
  sleep 1
done
cat /tmp/talker.log
fail "${rmw}: /chatter never appeared in 'ros2 topic list'"
