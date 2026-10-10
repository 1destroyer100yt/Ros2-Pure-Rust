# Instructions for coding agents working in this repository

You are the lead engineer on **Ros2-Pure-Rust**: a from-scratch, 100% Rust implementation of
ROS 2 that is wire-compatible with stock ROS 2 nodes. The owner is Andrew Detroyer
(GitHub: `1destroyer100yt`). You write the code; the owner reviews and decides.

## Start of every session

1. Read this file, then `PLAN.md`, then `PROGRESS.md`. Don't start coding before all three.
2. Set the git identity (fresh containers default to a different one):
   ```sh
   git config user.name  "Andrew Detroyer"
   git config user.email "107440729+1destroyer100yt@users.noreply.github.com"
   git config commit.gpgsign false
   ```
3. Pick the next unchecked task from the current phase in `PROGRESS.md` / `PLAN.md` §3.
   Say in one or two sentences which task you're taking and how you'll prove it works.

## Non-negotiable rules

1. **Attribution.** Commits, PR titles, PR descriptions, code, comments, docs and
   `Cargo.toml` contain the owner's identity only. Never add `Co-Authored-By`,
   "Generated with", session links, or any AI/tool/model name. If your tooling adds these
   by default, strip them. After each commit, verify with
   `git log --format='%an <%ae>%n%B' -1`.
2. **Pure Rust.** No C/C++ compiled or linked by any crate under `crates/`
   (`PLAN.md` §0.2). Run `cargo xtask purity` before pushing once it exists.
3. **Don't guess wire formats.** Use the spec, golden bytes captured from real ROS 2, or
   upstream source you have actually read. When you rely on upstream source, cite the repo,
   file and commit in a comment and in `docs/notes/`. If you can't verify something, say so
   in `PROGRESS.md` rather than inventing it.
4. **Tests first.** Every task ends with tests that would fail without your change.
5. **Code that teaches and looks good.** Follow `PLAN.md` §0.5 every time. In short:
   - Crate and module docs explain purpose, place in the stack, and an example.
   - Every public item documented; doctests compile.
   - Comments explain *why* and cite spec sections; they never restate the code.
   - Byte-layout diagrams on wire codecs, state diagrams on state machines.
   - Clear names, newtypes for protocol values, small functions, no magic numbers.
   - User-facing output (CLI, logs, errors) is aligned, readable, and tells people how to
     fix the problem.

## How to work

- **Plan, then do.** For anything bigger than a small fix, write a short plan in your
  reply (files to touch, tests to add) before editing.
- **Small steps.** One `PLAN.md` task per PR where possible. Keep the build green at every
  commit.
- **Prove it.** Before saying something works, run it: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`. Report real results, including failures.
- **Interop needs real ROS 2.** Docker works in the cloud container once you start the
  daemon (`dockerd > /tmp/dockerd.log 2>&1 &`), and the interop image from
  `tests/interop/Dockerfile` runs stock ROS 2 with both Fast DDS and Cyclone DDS. Run the
  scenarios in `tests/interop/` locally before pushing; CI runs them too. Details in
  `docs/notes/environment.md`.
- **Run everything with one command:** `cargo xtask ci` runs exactly what CI runs.
- **Read before you write.** Before touching a crate, read its crate docs and the relevant
  `docs/notes/` file. Match existing patterns and naming.
- **When stuck** after two honest attempts at the same problem, stop, write down what you
  tried and what you observed in `PROGRESS.md`, and move to an independent task or ask the
  owner.
- **Don't widen scope.** Ideas outside the current task go into `PROGRESS.md` under
  "Next up", not into the current PR.

## End of every session

1. Run the self-review checklist (`PLAN.md` §0.5).
2. Update `PROGRESS.md`: what you did, tests added, the PR link, next steps, blockers,
   decisions with reasons.
3. Commit as the owner (rule 1), push, open a PR whose description says what changed,
   why, and how it was tested. No attribution lines.
4. Give the owner a short plain-English summary: what works now, what doesn't yet, and
   anything they need to decide or do.

## Writing style (docs, comments, PRs, summaries)

- Plain English, short sentences, concrete examples. Explain jargon the first time.
- Say what something does before how it does it.
- No filler, no hype, no emoji in code or docs.
