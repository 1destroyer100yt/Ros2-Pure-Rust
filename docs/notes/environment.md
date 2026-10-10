# Development environment findings

## Docker and stock ROS 2 (checked 2026-10-10)

- The cloud coding container has the Docker client but the daemon is not running at
  start. Starting it works: `dockerd > /tmp/dockerd.log 2>&1 &`, then wait a few seconds.
- `ros:jazzy-ros-base` pulls and runs. It ships only Fast DDS (`rmw_fastrtps_cpp`).
- `tests/interop/Dockerfile` adds Cyclone DDS, the demo nodes and tcpdump. With it,
  `tests/interop/smoke.sh` passes locally for both `rmw_fastrtps_cpp` and
  `rmw_cyclonedds_cpp` (a C++ talker is discovered and its messages are received).
- So interop scenarios can run locally, not only in CI. Run them before pushing.
