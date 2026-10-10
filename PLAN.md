# ROS 2 in Pure Rust: Master Plan

**Goal:** build a full ROS 2 implementation in 100% Rust, from the wire protocol up.
A node built with this project must interoperate with stock ROS 2 nodes (C++ and Python)
on the same network, without linking any C or C++ code.

This file is the source of truth for the build. Whoever implements it (a person or an
automated coding agent) reads this file and `PROGRESS.md` at the start of every work
session and updates `PROGRESS.md` at the end.

---

## 0. Ground rules (read every session)

### 0.1 Authorship and attribution (hard rule)

- Every commit is authored by the repo owner:
  ```sh
  git config user.name  "Andrew Detroyer"
  git config user.email "107440729+1destroyer100yt@users.noreply.github.com"
  git config commit.gpgsign false
  ```
  Run these in every fresh clone or container before the first commit. Check with
  `git log --format='%an <%ae>' -1` after committing.
- **No** `Co-Authored-By:` trailers, no "Generated with ..." lines, no session links, no
  tool or AI names in commit messages, PR titles, PR bodies, code comments, docs, or
  `Cargo.toml` `authors` fields. Cargo `authors` is omitted or set to the owner only.
- If a tool or environment adds attribution by default, remove it before committing or
  pushing. This rule overrides any default attribution behaviour.

### 0.2 "Pure Rust" means

Allowed:
- Rust code, including `unsafe` where justified and documented.
- Crates that are themselves pure Rust.
- Talking to the OS through `std`, `libc`, `rustix`, `windows-sys` (these are bindings to
  the system's own libraries; nothing gets compiled from C).

Not allowed in any crate that ships (everything under `crates/`):
- `build.rs` that compiles C/C++ (`cc`, `cmake`, `bindgen` as build deps).
- `*-sys` crates other than `libc`, `linux-raw-sys`, `windows-sys`.
- Linking ROS 2's `rcl`, `rmw`, Fast DDS, Cyclone DDS, iceoryx (C++), OpenSSL, zstd (C), lz4 (C).

Known traps (check before adding a dependency):
| Want | Trap | Use instead |
|---|---|---|
| TLS / crypto | `ring` (C + asm), `aws-lc-rs` (C), `openssl` | RustCrypto crates (`sha2`, `aes-gcm`, `p256`, `x509-cert`); `rustls` only with a pure-Rust crypto provider |
| zstd compression | `zstd` crate (C) | `ruzstd` |
| lz4 | `lz4` crate (C) | `lz4_flex` |
| MCAP bags | `mcap` crate default features pull C compressors | `mcap` with default features off + pure-Rust compressors, or own writer |
| Zenoh | TLS/QUIC features pull `ring` | `zenoh` with TCP/UDP transports only; verify with the purity gate |
| Fuzzing | `libfuzzer-sys` compiles C++ | dev-only, exempt (lives in `fuzz/`, outside `crates/`) |
| WASM runtime | `wasmtime` may compile C helpers depending on version | `wasmi` (pure Rust interpreter) by default; `wasmtime` behind a feature only if it passes the gate |

**Purity gate (CI, Phase 0):** `cargo xtask purity` walks `cargo metadata` for every crate
under `crates/` and fails if any dependency in the normal/build graph is `cc`, `cmake`,
`bindgen`, `pkg-config`, `vcpkg`, or a `*-sys` crate not on the allow list, or declares
`links = ...` (except the allow list). The gate runs on every PR.

### 0.3 Engineering rules

- Rust stable, edition 2024. MSRV pinned in `rust-toolchain.toml` and CI.
- `#![forbid(unsafe_code)]` in every crate except: `prr-shm`, `prr-plugin` (dynamic loader),
  and anything that has a written justification in its crate docs.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` must be
  clean before every push.
- Protocol layers are **sans-IO**: they take bytes plus a clock and return bytes plus timer
  requests. Sockets and threads live in a separate layer. This makes everything testable
  without a network and deterministic in tests.
- **Never guess a wire format.** Every encoder and decoder gets golden byte vectors taken from
  the spec text or from packets captured from real ROS 2 (see §6). If a convention isn't
  documented, read the upstream source (`ros2/rmw_fastrtps`, `ros2/rmw_cyclonedds`,
  `ros2/rmw_dds_common`, `ros2/rosidl`, `eclipse-cyclonedds/cyclonedds`,
  `eProsima/Fast-DDS`) and record what you found in `docs/notes/<topic>.md` with the file
  and commit you read.
- Small PRs: one milestone task per PR where possible, each with tests.

### 0.4 Session protocol

1. Read `AGENTS.md`, `PLAN.md` (this file) and `PROGRESS.md`.
2. Take the first unchecked task in the current phase. Don't skip ahead unless blocked;
   if blocked, write the blocker in `PROGRESS.md` and take the next independent task.
3. Write the tests first (golden vectors / interop scenario), then the code.
4. Run the self-review checklist in §0.5 before committing.
5. Update `PROGRESS.md`: what was done, what's next, open questions, anything learned.
6. Commit with owner identity (§0.1), push, open a PR.

### 0.5 Code that explains itself and looks good

This codebase is meant to be read and learned from, not only run. Someone who knows
Rust but has never seen DDS should be able to open any file and follow it.

**Documentation**
- Every crate starts with a `//!` doc block covering what the crate does, where it sits in
  the stack (a small ASCII diagram is welcome), a short runnable example, and the spec
  sections or upstream files it implements.
- Every module starts with a `//!` paragraph saying what lives in it and why it's separate.
- Every public item has a `///` doc comment. CI enforces `#![deny(missing_docs)]` and
  `cargo doc` with `-D warnings` (no broken intra-doc links).
- Public functions document `# Errors`, `# Panics` (ideally "never"), and `# Safety` for
  anything `unsafe`. Doc examples are real doctests that compile and run.
- Comments explain **why**, never restate **what**. `// increment i` is noise;
  `// RTPS counts fragments from 1, not 0 (DDSI-RTPS 2.5 §8.3.7.3)` is useful.
- Every protocol detail cites its source: a spec section, or an upstream file and commit for
  ROS 2 conventions that are only defined by code (e.g.
  `// matches rmw_fastrtps_shared_cpp/src/rmw_request.cpp @ <commit>`).
- Wire formats get a byte-layout diagram above the encoder:
  ```rust
  /// ```text
  ///  0               1               2               3
  ///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
  /// +---------------+---------------+-------------------------------+
  /// | submessageId  |     flags     |       octetsToNextHeader      |
  /// +---------------+---------------+-------------------------------+
  /// ```
  ```
- State machines (reliable writer/reader, actions, lifecycle) get a state diagram in their
  module docs and a table of transitions.
- Spec acronyms (GUID, SPDP, SEDP, CDR, QoS, ...) are defined in `docs/GLOSSARY.md` and
  linked the first time they appear in each crate's docs.
- Examples in `examples/` read like tutorials: numbered steps in comments, explaining what
  each part does and what you should see when you run it.
- A user guide (`docs/book/`, built with mdBook, which is pure Rust) grows with each phase:
  getting started, concepts, how-tos, and "coming from rclcpp" side-by-side comparisons.

**Code shape**
- Clear names over short ones: `sequence_number`, not `sn`; `heartbeat_period`, not `hb_p`.
  Spec names are kept when they are the vocabulary readers will search for (`GuidPrefix`,
  `AckNack`), always with a doc comment.
- Small functions doing one thing; files roughly under 600 lines; one concept per module.
- Newtypes instead of bare integers for protocol values (`EntityId`, `SequenceNumber`,
  `DomainId`), so the type tells the reader what the number means.
- Make illegal states unrepresentable with enums and typestate instead of runtime checks
  and comments.
- No magic numbers: named constants with a doc comment and spec reference.
- Errors are typed (`thiserror`-style enums per crate) and their messages say what happened,
  why, and what to do about it.
- `rustfmt.toml` checked in; imports grouped std / external / crate; no commented-out code;
  no `TODO` without an issue or a `PROGRESS.md` entry.
- Selected `clippy::pedantic` lints on in `[workspace.lints]` (Phase 0 decides the list).

**Things users see should look good**
- CLI output: aligned tables, colour when attached to a terminal (respect `NO_COLOR`),
  `--json` on every listing command, and helpful errors with a hint line, e.g.
  ```text
  error: no publishers matched subscription on /scan
    why:  subscriber wants RELIABLE, publisher offers BEST_EFFORT
    fix:  use QoS profile `sensor_data`, or pass --qos-reliability best_effort
  ```
- Log output: readable default format with severity colours and node names, same env
  vars as ROS 2 for customising it.
- Panics carry context; library code never panics on bad network input.

**Self-review checklist (before every commit)**
- [ ] Could a newcomer understand each new public item from its docs alone?
- [ ] Is every protocol constant or layout cited?
- [ ] Any comment that restates the code? Delete it. Any non-obvious line without a "why"? Add one.
- [ ] `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, `cargo doc -D warnings`, purity gate all clean.
- [ ] `PROGRESS.md` updated; commit authored by the owner with no attribution trailers.

---

## 1. Target and compatibility

- **Primary target distro: ROS 2 Jazzy Jalisco (LTS).** Secondary: the newest LTS
  (Lyrical) and Kilted. At the start of Phase 1, diff the interface packages and rmw
  conventions between Jazzy and the newest LTS and write the result to
  `docs/notes/distro-diff.md`.
- **Interop targets:** `rmw_fastrtps_cpp` (default RMW in Jazzy), `rmw_cyclonedds_cpp`, and
  later `rmw_zenoh_cpp`.
- **Platforms:** Linux x86_64 and aarch64 first, then macOS and Windows. `no_std` + `alloc`
  for the core serialization/protocol crates so microcontrollers are possible later (Phase 11).

What "done" looks like for v1.0:
- `ros2 topic echo`, `ros2 node list`, `ros2 node info`, `ros2 param get/set/list`,
  `ros2 service call`, `ros2 action send_goal` all work against our nodes from a stock ROS 2
  install, and our CLI does the same against stock ROS 2 nodes.
- Our talker/listener, add_two_ints, and fibonacci action interop both ways with
  rclcpp and rclpy over Fast DDS and Cyclone DDS.
- Large messages (8 MB `sensor_msgs/Image`) work in both directions.
- Record/play bags readable by `ros2 bag` (MCAP).

---

## 2. Architecture

```
            ┌──────────────────────────────────────────────────────────┐
 user code  │ prr (client library: Node, Pub, Sub, Service, Action,    │
            │ Timer, Parameters, Executor, Lifecycle, Logging, Time)   │
            └───────────────┬──────────────────────────────────────────┘
                            │  Middleware trait (our own, NOT the C rmw ABI)
        ┌───────────────────┼─────────────────────┬────────────────────┐
        ▼                   ▼                     ▼                    ▼
  prr-dds (RTPS)      prr-zenoh            prr-loopback          prr-shm
  Fast DDS/Cyclone    rmw_zenoh compat     in-process,           same-host
  wire compatible     (Phase 8)            deterministic tests   zero-copy (Phase 9)
        │
  prr-rtps (sans-IO wire protocol)  ── prr-cdr (serialization)
                                       ▲
  prr-idl (parse .msg/.srv/.action/.idl, type hashes) → prr-codegen → prr-interfaces
```

### 2.1 Workspace layout

Crate prefix `prr-` ("pure Rust ROS") is a placeholder; rename before the first crates.io
publish if desired.

```
Cargo.toml                 workspace
rust-toolchain.toml
xtask/                     purity gate, codegen driver, golden-vector refresh, release
crates/
  prr-cdr/                 CDR / XCDR1 / XCDR2 encode+decode, derive macro (no_std+alloc)
  prr-cdr-derive/          proc macro
  prr-idl/                 .msg/.srv/.action parser, OMG IDL subset parser, TypeDescription, RIHS01 hash
  prr-codegen/             IDL → Rust source (library + CLI + build.rs helper)
  prr-interfaces/          generated Rust for vendored common interface packages
  prr-rtps/                DDSI-RTPS 2.5 messages, submessages, parameter lists (sans-IO, no_std+alloc)
  prr-dds/                 participants, discovery (SPDP/SEDP), writers/readers, reliability, QoS, transports
  prr-mw/                  Middleware trait + shared types (Gid, QoS profiles, graph events)
  prr-loopback/            in-process Middleware for deterministic tests
  prr/                     client library (the user-facing crate)
  prr-action/              actions
  prr-lifecycle/           managed nodes
  prr-tf2/                 tf2 buffer/listener/broadcaster + math
  prr-bag/                 MCAP record/play compatible with rosbag2
  prr-launch/              YAML/XML launch files + Rust launch API
  prr-cli/                 `prr` binary (ros2-CLI-like)
  prr-plugin/              plugin system (static, dynamic, WASM)
  prr-plugconv/            C++ pluginlib → Rust converter
  prr-zenoh/               Zenoh backend (Phase 8)
  prr-shm/                 shared-memory transport (Phase 9)
  prr-security/            DDS Security / SROS2 (Phase 10)
  prr-testkit/             interop harness, packet capture helpers, golden vectors
interfaces/                vendored .msg/.srv/.action/.idl files (Apache-2.0, keep upstream LICENSE + NOTICE)
examples/                  talker, listener, add_two_ints, fibonacci, parameters, lifecycle, tf
docs/notes/                findings from upstream source / spec reading
tests/interop/             docker-compose / CI scenarios against stock ROS 2
fuzz/                      fuzz targets (dev-only, exempt from purity gate)
```

---

## 3. Phases

Each phase lists tasks with acceptance criteria. Check them off in `PROGRESS.md`.

### Phase 0: Bootstrap

- [ ] Workspace skeleton, `rust-toolchain.toml`, `.gitignore`, empty crates compiling.
- [ ] `xtask purity` (see §0.2) with a test proving it catches a `cc` dependency.
- [ ] GitHub Actions: fmt, clippy, test, `cargo doc -D warnings`, purity on Linux. Matrix
      with macOS/Windows later.
- [ ] Code-quality baseline (§0.5): `rustfmt.toml`, `[workspace.lints]` with
      `missing_docs = "deny"` and the chosen pedantic lints, `docs/GLOSSARY.md`,
      `docs/book/` mdBook skeleton, crate-doc template used by every crate.
- [ ] Interop CI job skeleton: a job running in `ros:jazzy` (container) that can run
      `ros2 topic list`. This becomes the interop harness.
- [ ] `PROGRESS.md` created from the template in §9.
- [ ] License decision recorded (owner decides, see §8).

### Phase 1: Serialization and interfaces

**prr-cdr**
- [ ] Encapsulation header (CDR_BE 0x0000, CDR_LE 0x0001, and the XCDR2 variants for later).
- [ ] XCDR1 primitives with alignment measured from the end of the 4-byte encapsulation
      header; 8-byte alignment for 64-bit types in XCDR1.
- [ ] Strings: `u32` length that counts the trailing NUL, then the bytes, then NUL.
- [ ] Wide strings: document and test both Fast DDS and Cyclone behaviour (they've
      differed historically; confirm by reading source).
- [ ] Sequences (bounded/unbounded), fixed arrays, nested structs, bool as one byte.
- [ ] `#[derive(CdrEncode, CdrDecode)]`; zero-copy decode for `&[u8]`/`&str`
      where alignment allows.
- [ ] Property tests: roundtrip for random values of every generated type.

**prr-idl**
- [ ] `.msg` parser: field types, arrays `T[]`, `T[N]`, `T[<=N]`, bounded strings
      `string<=N`, constants, default values, comments.
- [ ] `.srv` (request `---` response), `.action` (goal `---` result `---` feedback).
- [ ] OMG IDL subset parser (the subset `rosidl` emits: modules, structs, typedefs,
      constants, sequences, annotations `@default`, `@verbatim`, `@key` ignored).
- [ ] Action expansion: `<Action>_Goal`, `_Result`, `_Feedback`, `_SendGoal`,
      `_GetResult`, `_FeedbackMessage` exactly as rosidl does it.
- [ ] Service expansion including the `_Event` message used for service introspection.
- [ ] `TypeDescription` builder and **RIHS01** type hash (SHA-256 over rosidl's canonical
      JSON of the type description). Golden test: for every vendored type, our hash equals
      the hash in the `share/<pkg>/msg/<Type>.json` files that a Jazzy install ships.

**prr-codegen + prr-interfaces**
- [ ] Rust output: `pkg::msg::Type`, `pkg::srv::Type{,_Request,_Response}`,
      `pkg::action::...`; constants as associated consts; defaults via `Default`.
- [ ] A `TypeSupport` trait per type: ROS type name (`std_msgs/msg/String`), DDS type name
      (`std_msgs::msg::dds_::String_`), type hash, CDR encode/decode.
- [ ] Vendor interface packages: `builtin_interfaces`, `std_msgs`, `std_srvs`,
      `geometry_msgs`, `sensor_msgs`, `nav_msgs`, `shape_msgs`, `trajectory_msgs`,
      `visualization_msgs`, `diagnostic_msgs`, `rcl_interfaces`, `action_msgs`,
      `unique_identifier_msgs`, `lifecycle_msgs`, `rosgraph_msgs`, `statistics_msgs`,
      `type_description_interfaces`, `service_msgs`, `tf2_msgs`, `rmw_dds_common`,
      `example_interfaces`, `action_tutorials_interfaces`.
- [ ] `build.rs` helper so user packages generate Rust from their own `.msg` files with no
      colcon/ament. Also reads `package.xml` dependencies.

**Acceptance:** byte-for-byte equality with payloads captured from rclcpp for a fixed set of
sample messages (generate the goldens once in the interop CI job and commit them under
`crates/prr-testkit/golden/`).

### Phase 2: RTPS wire protocol (`prr-rtps`, sans-IO)

Reference: OMG DDSI-RTPS 2.5.
- [ ] Core types: GuidPrefix, EntityId (incl. builtin IDs), GUID, SequenceNumber,
      SequenceNumberSet, FragmentNumberSet, Locator, Time, Count, VendorId, ProtocolVersion.
- [ ] Message header + submessages: DATA, DATA_FRAG, HEARTBEAT, HEARTBEAT_FRAG, ACKNACK,
      NACK_FRAG, GAP, INFO_TS, INFO_DST, INFO_SRC, INFO_REPLY, PAD. Endianness flag per
      submessage.
- [ ] ParameterList codec with all PIDs used by SPDP/SEDP, plus vendor-specific PIDs from
      Fast DDS and Cyclone parsed or skipped safely (unknown PIDs must be skipped,
      not rejected).
- [ ] Inline QoS, including `PID_RELATED_SAMPLE_IDENTITY` (Fast DDS service correlation)
      and the key hash / status info.
- [ ] Fuzz every decoder (`fuzz/`). Decoders must never panic on garbage.

**Acceptance:** decode and re-encode every packet in a corpus captured from Fast DDS and
Cyclone (`tests/interop/pcaps/`), with semantic equality.

### Phase 3: DDS layer (`prr-dds`)

- [ ] UDPv4 transport: unicast + multicast, port mapping (PB=7400, DG=250, PG=2,
      d0=0, d1=10, d2=1, d3=11), default multicast group 239.255.0.1. Honour
      `ROS_DOMAIN_ID`, `ROS_AUTOMATIC_DISCOVERY_RANGE`, `ROS_STATIC_PEERS`, and the legacy
      `ROS_LOCALHOST_ONLY`. UDPv6 after.
- [ ] SPDP: participant announcements, lease duration, liveliness, removal on timeout.
      Participant USER_DATA `enclave=/;` as ROS 2 sets it.
- [ ] SEDP: publications/subscriptions writers/readers (reliable, transient local),
      endpoint discovery and matching, endpoint USER_DATA `typehash=RIHS01_<hex>;`.
- [ ] Best-effort writer/reader.
- [ ] Reliable stateful writer/reader: heartbeats, ACKNACK, retransmit, GAP, history depth,
      KEEP_LAST/KEEP_ALL, resource limits.
- [ ] Fragmentation: DATA_FRAG send and reassembly, NACK_FRAG, HEARTBEAT_FRAG. Default
      fragment size that fits a 64 KB UDP datagram; configurable.
- [ ] QoS: reliability, durability (volatile, transient local), history, depth, deadline,
      lifespan, liveliness (automatic, manual by topic), lease duration. Request/offered
      compatibility matching plus incompatible-QoS events.
- [ ] Unkeyed topics (ROS 2 messages have no keys). Keyed support is optional, later.
- [ ] Builtin participant message (liveliness) reader/writer.

**Acceptance (interop CI):** our raw DDS writer publishing `rt/chatter` with type
`std_msgs::msg::dds_::String_` is received by `ros2 topic echo /chatter` on Fast DDS and
Cyclone, and the reverse direction works too. Late-joining transient-local subscriber gets
history. 8 MB message works both ways.

### Phase 4: Client library core (`prr`)

ROS 2 conventions to implement exactly (verify each against upstream source, record in
`docs/notes/`):
- Topic mangling: `/foo` → DDS topic `rt/foo`; service request `rq/<srv>Request`, reply
  `rr/<srv>Reply`. Type names `pkg::msg::dds_::Type_`, `pkg::srv::dds_::Type_Request_`.
- **One DDS participant per Context**, many nodes per participant. Node ↔ endpoint
  mapping is published on `ros_discovery_info` (`rmw_dds_common/msg/ParticipantEntitiesInfo`,
  reliable, transient local, keep last 1). Without this, `ros2 node list` won't see us.
- Service request/reply correlation differs per vendor:
  - Fast DDS: `PID_RELATED_SAMPLE_IDENTITY` in inline QoS on the reply, and the request's
    writer GUID + sequence number as the sample identity.
  - Cyclone: a request header prepended to the payload (client GUID + sequence number).
  Detect the remote vendor from the participant's VendorId and speak its convention. Confirm
  the exact layouts by reading the rmw sources before coding.

Tasks:
- [ ] `Context` (init from args/env), `Node` (name, namespace, validation rules identical to
      rcl), `Publisher<T>`, `Subscription<T>`, `Timer`, `GuardCondition`, `WaitSet`.
- [ ] Executors: single-threaded and multi-threaded (callback groups: mutually exclusive
      and reentrant), plus an async API (`async fn` handlers, `Stream` subscriptions) that
      works without tokio. Optional `tokio` feature for integration.
- [ ] Command line: `--ros-args`, `-r` remapping (incl. `__node`, `__ns`), `-p`,
      `--params-file` (same YAML format as ROS 2), `--log-level`, `--enclave`.
- [ ] Parameters: declare/get/set with descriptors and ranges, set callbacks, the six
      parameter services (`~/get_parameters`, `~/set_parameters`, ...), and
      `/parameter_events` publishing.
- [ ] Time: system, steady, ROS time; `use_sim_time` with `/clock` subscription; rate,
      duration, timers on ROS time.
- [ ] Logging: `info!/warn!/...` macros, severities, throttling, `/rosout` publishing
      (`rcl_interfaces/msg/Log`, transient local), console format env vars.
- [ ] Graph API: node names, topic names and types, publishers/subscribers info by topic,
      graph change events.
- [ ] `prr-loopback` middleware so every client-library test runs in-process with no network.

**Acceptance:** our `talker`/`listener` interop with `demo_nodes_cpp` and `demo_nodes_py`;
`ros2 node list`, `ros2 node info`, `ros2 param list/get/set/dump` work on our nodes.

### Phase 5: Services, actions, lifecycle

- [ ] `Client<S>` (async `call`, timeouts, `wait_for_service`), `Service<S>`.
- [ ] Service introspection (`_service_event` topics) off by default, configurable.
- [ ] Actions (`prr-action`): server and client, goal state machine, cancel, feedback,
      result caching with timeout, `_action/status` topic. Goal UUIDs.
- [ ] Lifecycle nodes (`prr-lifecycle`): state machine, transition services, lifecycle
      publishers that drop messages when inactive, `transition_event` topic.
- [ ] Content-filtered topics (if the target RMW supports it; best effort).

**Acceptance:** `add_two_ints` and `fibonacci` interop both directions with C++ and Python
on both DDS vendors; `ros2 lifecycle set` drives our managed node.

### Phase 6: Tooling

- [ ] `prr` CLI (`prr-cli`): `topic list|echo|pub|hz|bw|info|type`, `node list|info`,
      `service list|call|type`, `action list|send_goal|info`, `param ...`,
      `interface show|list`, `bag record|play|info`, `run`, `launch`, `doctor`.
      Dynamic messages: echo/pub any type by loading its definition at runtime
      (via `type_description_interfaces` services or local `.msg` files).
- [ ] `prr-bag`: MCAP record/play compatible with `ros2 bag` (rosbag2 metadata.yaml +
      MCAP schema/channel conventions); pure-Rust compression only.
- [ ] `prr-launch`: XML and YAML launch formats (the same schemas ROS 2 uses), plus a Rust
      API. Python launch files are **not** supported (no Python interpreter). Provide a
      `prr launch convert` that turns simple Python launch files into YAML by pattern
      matching, flagging what it can't convert.
- [ ] `prr-tf2`: buffer with time interpolation (slerp), static/dynamic broadcasters,
      listener, `lookup_transform` with timeouts, message filters.

### Phase 7: Plugins and the C++ plugin converter

See §4 for the design. Tasks:
- [ ] `prr-plugin` static registry (compile-time, `linkme` distributed slices).
- [ ] Dynamic loader (C-ABI vtable + version handshake, `libloading`).
- [ ] WASM plugins (`wasmi`), with a host API for logging, params, pub/sub.
- [ ] pluginlib-compatible manifest reader (`plugins.xml`).
- [ ] `prr-plugconv` MVP + mapping tables for 3 base classes, with 5 sample C++ plugins
      written in this repo as golden tests.

### Phase 8: Zenoh backend (`prr-zenoh`)

- [ ] Implement the `rmw_zenoh` conventions: key expressions
      (`<domain_id>/<fqn>/<type_name>/<type_hash>`), liveliness tokens for graph
      discovery, attachments (sequence number, source timestamp, GID), queryables for
      services. Read the `ros2/rmw_zenoh` design doc and source and pin a version.
- [ ] Works with and without a `zenohd` router (our CLI can start a pure-Rust router).

**Acceptance:** interop with `rmw_zenoh_cpp` nodes for topics, services, actions.

### Phase 9: Performance

- [ ] Intra-process zero-copy (pass `Arc<T>` / owned message directly when pub and sub are
      in the same process).
- [ ] Shared-memory transport between processes on the same host (`prr-shm`), own
      implementation or `iceoryx2` (pure Rust; run it through the purity gate).
- [ ] Loaned messages API.
- [ ] Benchmarks (`criterion`): latency and throughput vs rclcpp for 64 B, 4 KB, 1 MB, 8 MB
      payloads. Run them on dedicated hardware; cloud CI numbers are noise.

### Phase 10: Security (SROS2 / DDS Security)

- [ ] Authentication (PKI-DH), access control (governance/permissions XML signed with
      S/MIME), cryptographic (AES-GCM) plugins per DDS Security 1.1, using RustCrypto.
- [ ] Compatible with `ros2 security` generated keystores.
- **Needs human expert review before anyone relies on it.** Mark the crate experimental
  until audited.

### Phase 11: Microcontrollers

- [ ] `no_std` build of `prr-cdr`, `prr-rtps`, a minimal best-effort `prr-dds` subset, and
      a tiny client API. Target `embassy` on RP2040/ESP32-C3/STM32 with `smoltcp`/
      `embassy-net`. Talks RTPS directly to the ROS 2 graph: no micro-ROS agent needed.

### Phase 12: Ecosystem (post-1.0, separate repos)

`ros2_control`-style controller framework, navigation stack building blocks,
`robot_state_publisher`, `joint_state_publisher`, URDF parser, image transport. Each
uses the plugin system from Phase 7.

---

## 4. Plugins

### 4.1 Can Rust do plugins?

Yes. The real limitation is that **Rust has no stable ABI**, so you can't load a `.so` built
by a different compiler version and call Rust trait objects across it. ROS 2's
`pluginlib` works because C++ has a de facto stable ABI per platform. We support three
styles, in order of preference:

1. **Static plugins (default).** Plugins are normal crates. Each registers itself under a
   string name (same names as pluginlib, e.g. `my_pkg/MyPlanner`) in a compile-time
   registry. The node picks one by name from a parameter at runtime, which is the same
   user experience as pluginlib, with full type safety and no ABI problems. You choose
   *which plugins are available* at build time and *which one runs* at runtime.
2. **Dynamic plugins.** `.so`/`.dll`/`.dylib` exporting one `extern "C"` function that
   returns a `#[repr(C)]` vtable plus a version handshake (plugin API version, `prr` version,
   rustc version). The loader refuses mismatches instead of crashing. This is the only
   `unsafe`-heavy area and gets careful tests.
3. **WASM plugins.** Sandboxed, hot-reloadable, can be written in any language that targets
   WASM, can't crash the host. Slower; good for user scripts, behaviours, filters.

### 4.2 `prr-plugconv`: C++ plugin → Rust converter

A complete C++ → Rust translator is an unsolved research problem, so this tool is
deliberately simple. It's a skeleton generator plus a pattern-based translator that
**always produces a compiling crate** and marks everything it isn't sure about.

Input: a C++ ROS 2 package directory (`package.xml`, `plugins.xml`, `CMakeLists.txt`,
`include/**/*.hpp`, `src/**/*.cpp`).

Pipeline:
1. **Manifest:** read `plugins.xml` → list of `(class type, base_class_type, library)`.
2. **Base-class mapping:** look up `base_class_type` in mapping tables
   (`crates/prr-plugconv/mappings/*.toml`, user-extensible). Each table maps the C++
   base class to a Rust trait and maps each virtual method to a trait method, e.g.
   `nav2_core::GlobalPlanner::createPlan` → `GlobalPlanner::create_plan`. Base classes we
   don't have a trait for get a generated trait with the same methods.
3. **Skim parse:** a small hand-written C++ lexer + brace matcher (not a compiler) finds the
   class, member fields, method bodies, `declare_parameter` / `get_parameter` calls,
   `create_publisher` / `create_subscription` calls, and `#include`s. Everything it doesn't
   recognise is kept as raw text.
4. **Type mapping table:** `double`→`f64`, `int`→`i32`, `std::string`→`String`,
   `std::vector<T>`→`Vec<T>`, `std::shared_ptr<T>`→`Arc<T>`,
   `rclcpp::Publisher<M>::SharedPtr`→`prr::Publisher<M>`, `geometry_msgs::msg::Pose`→
   `geometry_msgs::msg::Pose`, and so on.
5. **Line rewrite rules** for a known subset: `RCLCPP_INFO(logger, "x=%d", x)` →
   `info!(logger, "x={}", x)`, `auto x = ...;` → `let x = ...;`, `this->` / `->` member
   access → `self.`/`.`, range-for loops, `std::sqrt(x)` → `x.sqrt()`, `std::min/max`,
   `nullptr` checks → `Option`. Every rule has a test.
6. **Everything else** becomes:
   ```rust
   // plugconv: could not translate:
   // <original C++ line(s)>
   todo!("plugconv: translate line 42 of planner.cpp");
   ```
   so the crate compiles and fails loudly at runtime until a human (or an assistant)
   finishes it.
7. **Outputs:** a new crate (`Cargo.toml`, `src/lib.rs`, registration macro, parameter
   YAML), and `CONVERSION_REPORT.md` with the percentage of lines translated, a list of
   every `todo!()` with the original C++ next to it, and the parameter names found.
8. **`plugconv bundle`:** writes one self-contained Markdown file (C++ source, generated
   Rust, the target trait docs, the TODO list) that can be handed to any reviewer or coding
   assistant to finish the port. No network calls from the tool itself.

Golden tests: sample C++ plugins written in this repo (a planner, a costmap-layer-like
filter, a controller, a hardware-interface-like plugin, a pathological file full of
templates and macros). We check in the expected output and diff against it.

### 4.3 What makes this better than existing ROS-in-Rust projects

Existing projects mostly bind to the C `rcl` (needs a full ROS 2 install and colcon) or
are pure-Rust DDS clients without the rest of ROS 2. This project aims for:
- **Just `cargo build`.** No ROS install, no `source setup.bash`, no colcon, no CMake.
  Static single binaries, easy cross-compilation.
- **`prr doctor` that explains problems in plain English:** "your subscriber is RELIABLE
  but the publisher on `/scan` is BEST_EFFORT, so they won't connect"; "node `/foo` is on
  domain 3, you're on 0"; "multicast is blocked on this interface, set ROS_STATIC_PEERS".
- **Deterministic testing:** the loopback middleware plus a virtual clock lets you unit-test
  whole multi-node systems with no network and no flakiness.
- **Compile-time checked types** for topics, services, and parameters.
- **Microcontrollers on the same codebase** (Phase 11), no agent process.
- **Memory safety** across the whole stack including the network parser, which is fuzzed.
- **Plugin options C++ doesn't have:** sandboxed, hot-reloadable WASM plugins.

---

## 5. Things the implementing agent can't do (or can't do reliably)

Plan around these from the start:

| Limitation | Impact | Workaround in this plan |
|---|---|---|
| No native ROS 2 install in the coding container; the Docker daemon is not running at start | Interop tests need setup | Start `dockerd` and use the image in `tests/interop/` (verified working, see `docs/notes/environment.md`). CI runs the same image. UDP multicast inside one container works; across hosts it may not, so tests stay on one machine or use `ROS_STATIC_PEERS`. |
| Can't read the OMG spec PDFs if network access is restricted | Risk of guessing wire formats | Golden packets captured from real ROS 2 in CI are the ground truth; spec notes go in `docs/notes/`. Owner can drop spec PDFs into `docs/specs/` (don't commit them if their license forbids it). |
| No real robots, sensors, GPUs, displays | Can't test hardware drivers, rviz, Gazebo | Out of scope for v1. rviz2 and Gazebo keep working against our nodes because they speak the same wire protocol. |
| Cloud CI timing is noisy | Latency/throughput numbers meaningless | Benchmarks only produce relative numbers in CI; real benchmarking is an owner task on real hardware. |
| Can't judge crypto safety | Security plugins could be subtly broken | Phase 10 is marked experimental until a human audit. |
| Can't run Python launch files | `.launch.py` unsupported | XML/YAML launch + converter (Phase 6). |
| Limited context per session; project is large | Lost decisions, repeated work | `PROGRESS.md` + `docs/notes/` + small PRs. Every session starts by reading them. |
| Can't publish to crates.io, can't change repo settings or secrets | Release/CI setup needs owner | Owner tasks list in §8. |
| Can't make legal decisions | License choice | Owner decides (§8). |
| Full C++ → Rust translation isn't possible | Plugin converter is partial | By design: compiling skeleton + `todo!()` + report + bundle (§4.2). |
| Long soak tests (days) won't fit in a session | Rare reliability bugs | Nightly CI soak job (owner enables schedule), plus fuzzing and property tests. |

---

## 6. Testing strategy

1. **Unit + property tests** in every crate (`proptest`).
2. **Golden vectors:** CDR payloads and RTPS packets captured from stock ROS 2 in CI
   (`tcpdump` in the `ros:jazzy` container), committed under `prr-testkit/golden/` with a
   script `cargo xtask goldens` that regenerates them.
3. **Loopback integration tests:** multi-node scenarios in one process on a virtual clock.
4. **Interop CI matrix:** {our node} × {rclcpp, rclpy} × {Fast DDS, Cyclone, later Zenoh} ×
   {topic, service, action, params, large message, transient local, best effort}, both
   directions. Each scenario is a script under `tests/interop/` with a timeout and a clear
   pass/fail line.
5. **Fuzzing** of CDR, RTPS, ParameterList, IDL parser, launch parser.
6. **Purity gate** on every PR.

---

## 7. Reference sources

- OMG DDSI-RTPS 2.5, DDS 1.4, DDS-XTypes 1.3, DDS Security 1.1.
- design.ros2.org (topic/service name mangling, node-to-participant mapping, QoS).
- Upstream source to read when a convention is unclear: `ros2/rmw_fastrtps`,
  `ros2/rmw_cyclonedds`, `ros2/rmw_dds_common`, `ros2/rmw_zenoh`, `ros2/rcl`,
  `ros2/rclcpp`, `ros2/rosidl`, `ros2/rosidl_typesupport_fastrtps`, `ros2/rosbag2`,
  `ros2/launch`, `ros/pluginlib`.
- Existing Rust projects to learn from (don't copy code without checking licenses):
  RustDDS, ros2-client, r2r, ros2_rust (rclrs), zenoh, iceoryx2, mcap.

---

## 8. Owner decisions and tasks

- [ ] **License.** Suggested: Apache-2.0 (same as ROS 2; vendored interface files are
      Apache-2.0 already). Alternatively dual MIT/Apache-2.0, common in Rust.
- [ ] Final crate name prefix (placeholder `prr-`).
- [ ] Enable GitHub Actions; allow scheduled workflows for nightly soak/fuzz.
- [ ] crates.io publishing token (only when ready to release).
- [ ] Real-hardware benchmarking runs.
- [ ] Security audit before Phase 10 is called stable.

---

## 9. `PROGRESS.md` template

```markdown
# Progress

## Current phase
Phase N: <name>

## Last session
- Date:
- Did:
- Tests added:
- PR:

## Next up
- [ ] ...

## Blockers / open questions
- ...

## Environment findings
- Docker works in container: yes/no
- Interop CI status:

## Decisions log
- YYYY-MM-DD: <decision> — <why>
```
