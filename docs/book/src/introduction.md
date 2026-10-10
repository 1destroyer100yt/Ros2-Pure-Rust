# Introduction

This guide explains how to use pure-Rust ROS 2 and how it works inside. It grows with each
phase of the plan.

Nothing is usable yet: the project is in Phase 0 (project setup). The sections below arrive
as the code does:

- **Getting started** (Phase 4): a talker and listener that talk to stock ROS 2 nodes.
- **Concepts**: nodes, topics, services, actions, parameters, QoS.
- **How-tos**: generating message types, writing plugins, recording bags.
- **Coming from rclcpp**: the same programs written both ways, side by side.

To read this guide locally, install mdBook (`cargo install mdbook`) and run
`mdbook serve docs/book`.
