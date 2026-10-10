# Glossary

Terms used across the code and docs, in plain English. Crate docs link here the first time
they use one of these. Keep entries short; link to the spec for the full story.

| Term | Meaning |
|---|---|
| **ACKNACK** | RTPS submessage a reliable reader sends to say which samples it has and which it is missing. |
| **Action** | ROS 2 pattern for long-running tasks: send a goal, get feedback while it runs, then a result. Built from services and topics. |
| **CDR** | Common Data Representation: the byte layout DDS uses to put a message on the wire (primitive sizes, alignment, byte order). |
| **XCDR1 / XCDR2** | Versions 1 and 2 of extended CDR, defined in DDS-XTypes. ROS 2 uses XCDR1 ("plain CDR") by default. |
| **Encapsulation header** | The 4 bytes in front of every CDR payload saying which representation and byte order follow. |
| **DDS** | Data Distribution Service: the OMG publish/subscribe standard ROS 2's default middlewares implement. |
| **DDSI-RTPS / RTPS** | Real-Time Publish-Subscribe: the wire protocol DDS implementations use to talk to each other. This is what makes Fast DDS, Cyclone DDS and our code interoperable. |
| **Domain / domain ID** | A numbered, isolated DDS network. Nodes only see each other on the same domain (`ROS_DOMAIN_ID`, default 0). The ID also picks the UDP ports. |
| **Participant** | A DDS application's membership in one domain. ROS 2 creates one per context, shared by all nodes in it. |
| **Endpoint** | A DDS writer or reader; ROS publishers, subscriptions, clients and services are built from them. |
| **GUID** | 16-byte globally unique ID for every RTPS entity: a 12-byte participant prefix plus a 4-byte entity ID. |
| **Discovery** | How participants and endpoints find each other. RTPS splits it into SPDP and SEDP. |
| **SPDP** | Simple Participant Discovery Protocol: periodic multicast announcements of each participant. |
| **SEDP** | Simple Endpoint Discovery Protocol: participants exchange their writers and readers with types and QoS, so matching ones can connect. |
| **HEARTBEAT** | RTPS submessage a reliable writer sends to say which samples it has available. |
| **Fragmentation** | Splitting a message bigger than one UDP datagram into DATA_FRAG submessages. |
| **QoS** | Quality of Service: settings such as reliability, durability and history depth. A publisher and subscription only connect if their QoS is compatible. |
| **Reliable / best effort** | Reliable retransmits lost samples; best effort does not. |
| **Volatile / transient local** | Volatile: late joiners get nothing from the past. Transient local: a writer keeps recent samples and sends them to late joiners. |
| **RMW** | ROS Middleware interface: the C API in ROS 2 that hides which middleware is used. Our equivalent is the `prr-mw` Rust trait. |
| **rcl / rclcpp / rclpy** | ROS 2's C core library and its C++ and Python client libraries. `prr` plays the role of rclcpp/rclpy. |
| **rosidl** | ROS 2's interface definition tooling: parses `.msg`/`.srv`/`.action` files and generates code. |
| **TypeDescription / RIHS01** | A structured description of a message type, and the SHA-256-based hash of it that ROS 2 (Iron and later) advertises so mismatched types are detected. |
| **Sans-IO** | Code that only transforms data (bytes in, bytes out) and never touches sockets, threads or clocks itself. Makes protocol code deterministic and easy to test. |
| **Golden vector** | Bytes captured from real ROS 2 and stored in the repo; our encoder must produce them exactly and our decoder must read them. |
| **Interop** | Interoperability: our nodes and stock ROS 2 nodes exchanging data correctly. |
| **pluginlib** | ROS 2's C++ library for loading classes by name from shared libraries at runtime. |
| **Zenoh** | A pub/sub protocol; `rmw_zenoh` is a ROS 2 middleware built on it. |
| **MCAP** | The file format `ros2 bag` uses by default to record messages. |
| **SROS2** | ROS 2's tooling for DDS Security: authentication, access control and encryption. |
| **Purity gate** | `cargo xtask purity`: the check that no shipped crate compiles or links C/C++. |
