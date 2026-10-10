# Progress

## Current phase
Phase 0: Bootstrap (nearly done; only the owner's license decision is left)

## Last session
- Date: 2026-10-10
- Did:
  - Workspace with all 22 crates from PLAN.md §2.1 as documented skeletons that compile
    (`publish = false` until a license is chosen).
  - `cargo xtask purity`: walks `cargo metadata` from every crate under `crates/` and fails
    on native build tooling, `-sys` crates (except OS bindings) and `links` keys.
  - `cargo xtask ci`: runs fmt, clippy, tests, docs and purity, the same as CI.
  - Code-quality baseline: `rustfmt.toml`, `clippy.toml`, `[workspace.lints]`
    (missing docs denied, clippy pedantic, no `todo!`/`dbg!`), `docs/GLOSSARY.md`,
    `docs/CRATE_DOCS.md` template, `docs/book/` mdBook skeleton.
  - GitHub Actions: `ci.yml` (checks + MSRV 1.85) and `interop.yml` (smoke test against
    stock ROS 2 for Fast DDS and Cyclone DDS).
  - Interop image `tests/interop/Dockerfile` and `tests/interop/smoke.sh`.
- Tests added:
  - 8 purity-gate unit tests (clean graph, direct and transitive `cc`, dev-deps ignored,
    tools outside `crates/` ignored, `-sys` crates, `links`, malformed metadata).
  - Manually confirmed: adding `cc` as a build dependency of `prr-dds` fails the gate
    with the dependency path printed; removing it passes again.
  - Interop smoke passes locally for `rmw_fastrtps_cpp` and `rmw_cyclonedds_cpp`.
- PR: phase-0-bootstrap (see GitHub)

## Phase 0 checklist
- [x] Workspace skeleton, toolchain file, .gitignore, crates compiling
- [x] `xtask purity` with tests proving it catches `cc`
- [x] GitHub Actions: fmt, clippy, test, doc, purity (Linux) + MSRV
- [x] Code-quality baseline (rustfmt, lints, glossary, mdBook, crate-doc template)
- [x] Interop CI job skeleton (`ros2 topic list` and echo against a stock talker)
- [x] PROGRESS.md from template
- [ ] License decision (owner)

## Next up
- [ ] Phase 1, prr-cdr: encapsulation header + XCDR1 primitives, with golden vectors
- [ ] Interop: a `capture-goldens.sh` scenario that records CDR payloads and RTPS packets
      from stock nodes with tcpdump into `crates/prr-testkit/golden/` (needed by Phase 1
      and Phase 2 acceptance tests)
- [ ] `docs/notes/distro-diff.md`: Jazzy vs newest LTS differences (PLAN.md §1)
- [ ] Later: macOS/Windows CI matrix; mdBook build in CI

## Blockers / open questions
- License not chosen yet (PLAN.md §8). Crates are `publish = false` until then.
- GitHub Actions must be enabled on the repository for CI to run.

## Environment findings
- Docker works in container: yes, after starting `dockerd` (docs/notes/environment.md)
- Base `ros:jazzy-ros-base` has Fast DDS only; our interop image adds Cyclone DDS
- Interop CI status: workflow added; first run on the Phase 0 PR

## Decisions log
- 2026-10-10: target Jazzy LTS first, interop with Fast DDS and Cyclone DDS before Zenoh.
- 2026-10-10: purity gate allows `linux-raw-sys` and `windows-sys` (OS bindings, no C
  compiled); `libc` is not a `-sys` crate and passes without an exception.
- 2026-10-10: clippy pedantic on, minus `module_name_repetitions` and
  `must_use_candidate` (noise without catching bugs).
- 2026-10-10: `QoS`, `ROS`, `DDS`, `RTPS` are prose in docs (`clippy.toml`
  `doc-valid-idents`), not code identifiers.
- 2026-10-10: interop runs in one Docker image used by both CI and local runs, so results
  match.
