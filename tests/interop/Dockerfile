# Stock ROS 2 Jazzy plus what the interop scenarios need. Used both by CI
# (.github/workflows/interop.yml) and locally (tests/interop/README.md), so a scenario
# behaves the same in both places.
FROM ros:jazzy-ros-base

# - rmw_cyclonedds_cpp: the second DDS vendor we must interoperate with (Fast DDS ships in
#   the base image and is the Jazzy default).
# - demo_nodes_cpp/py, action_tutorials_cpp: the reference talker/listener, add_two_ints
#   and fibonacci nodes named in PLAN.md §1.
# - tcpdump: captures golden RTPS packets (PLAN.md §6).
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      ros-jazzy-rmw-cyclonedds-cpp \
      ros-jazzy-demo-nodes-cpp \
      ros-jazzy-demo-nodes-py \
      ros-jazzy-action-tutorials-cpp \
      tcpdump \
 && rm -rf /var/lib/apt/lists/*
