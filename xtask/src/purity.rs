//! `cargo xtask purity`: the gate that keeps shipped crates pure Rust (PLAN.md §0.2).
//!
//! It reads the resolved dependency graph from `cargo metadata`, starts from every
//! workspace crate under `crates/`, follows normal and build dependencies (dev-dependencies
//! never ship, so they are skipped), and reports any package that would compile or link
//! native C/C++ code:
//!
//! - known native build tooling (`cc`, `cmake`, `bindgen`, ...),
//! - `*-sys` crates, which by convention bind a native library,
//! - any package declaring `links = "..."` in its manifest.
//!
//! The graph is resolved for every target platform at once, so a Windows-only native
//! dependency fails the gate on Linux too.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

/// What counts as impure, and the reviewed exceptions.
#[derive(Debug)]
pub struct Policy {
    /// Crates whose only job is building or locating native code.
    pub banned_crates: &'static [&'static str],
    /// `-sys` crates that bind the operating system's own interface rather than compiling
    /// C code (PLAN.md §0.2, "Allowed").
    pub allowed_sys_crates: &'static [&'static str],
    /// Packages allowed to declare `links`. Each addition needs a comment saying why it is
    /// still pure Rust.
    pub allowed_links: &'static [&'static str],
}

/// The policy CI enforces.
pub const DEFAULT_POLICY: Policy = Policy {
    banned_crates: &["cc", "cmake", "bindgen", "pkg-config", "vcpkg"],
    // linux-raw-sys: raw Linux syscall definitions written in Rust, no C compiled.
    // windows-sys: declarations of Windows system APIs, no C compiled.
    allowed_sys_crates: &["linux-raw-sys", "windows-sys"],
    allowed_links: &[],
};

/// Why a package fails the gate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reason {
    /// The package is native build tooling such as `cc`.
    NativeBuildTool,
    /// The package name ends in `-sys` and is not on the allow list.
    SysCrate,
    /// The package declares `links = "<library>"`.
    LinksNativeLibrary(String),
}

/// One shipped crate reaching one impure package. Ordered by path, so output is sorted
/// and the same finding reached twice collapses into one.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    /// Dependency path from the shipped crate to the offender, as `name version` strings.
    pub path: Vec<String>,
    /// Name of the offending package (the last element of `path`).
    pub package: String,
    /// What is wrong with it.
    pub reason: Reason,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let root = self.path.first().map_or("?", String::as_str);
        let why = match &self.reason {
            Reason::NativeBuildTool => {
                format!(
                    "`{}` is native build tooling: it compiles or locates C/C++ code",
                    self.package
                )
            }
            Reason::SysCrate => {
                format!(
                    "`{}` is a `-sys` crate, which binds a native C library",
                    self.package
                )
            }
            Reason::LinksNativeLibrary(library) => {
                format!(
                    "`{}` declares `links = \"{library}\"`, so it links native code",
                    self.package
                )
            }
        };
        writeln!(f, "error: `{root}` depends on `{}`", self.package)?;
        writeln!(f, "  why:   {why}")?;
        writeln!(f, "  path:  {}", self.path.join(" -> "))?;
        write!(
            f,
            "  fix:   use a pure-Rust alternative (PLAN.md §0.2, \"Known traps\"); if the \
             package really is pure Rust, add it to the allow list in xtask/src/purity.rs \
             with a comment saying why"
        )
    }
}

/// The outcome of a check.
#[derive(Debug)]
pub struct Report {
    /// Number of shipped crates (workspace members under `crates/`).
    pub crates_checked: usize,
    /// Number of distinct packages reachable from them, the crates included.
    pub packages_reached: usize,
    /// Everything that failed, sorted for stable output.
    pub violations: Vec<Violation>,
}

/// `cargo metadata` output did not have the shape we expect.
#[derive(Debug, PartialEq, Eq)]
pub struct MetadataError {
    /// The JSON field that was missing or had the wrong type.
    pub field: &'static str,
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error: unexpected `cargo metadata` output: missing or malformed `{}`\n  \
             fix:   check that `cargo metadata --format-version 1` runs, and update \
             xtask/src/purity.rs if its format changed",
            self.field
        )
    }
}

/// Entry point for `cargo xtask purity`.
///
/// # Errors
///
/// Returns a printable message if `cargo metadata` fails or any violation is found.
pub fn run() -> Result<(), String> {
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1"])
        .output()
        .map_err(|error| format!("error: could not run `cargo metadata`: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "error: `cargo metadata` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("error: `cargo metadata` printed invalid JSON: {error}"))?;
    let report = check(&metadata, &DEFAULT_POLICY).map_err(|error| error.to_string())?;

    if report.violations.is_empty() {
        println!(
            "purity: ok ({} crates under crates/, {} packages reached, no native code)",
            report.crates_checked, report.packages_reached
        );
        return Ok(());
    }
    let details: Vec<String> = report.violations.iter().map(ToString::to_string).collect();
    Err(format!(
        "{}\n\npurity: {} violation(s) in {} crates under crates/",
        details.join("\n\n"),
        report.violations.len(),
        report.crates_checked
    ))
}

/// A package as far as the gate cares.
struct Package<'a> {
    name: &'a str,
    version: &'a str,
    manifest_path: &'a str,
    links: Option<&'a str>,
}

impl Package<'_> {
    fn label(&self) -> String {
        format!("{} {}", self.name, self.version)
    }
}

/// Checks parsed `cargo metadata --format-version 1` output against `policy`.
///
/// # Errors
///
/// Returns [`MetadataError`] if a field the gate relies on is missing.
pub fn check(metadata: &Value, policy: &Policy) -> Result<Report, MetadataError> {
    let packages = parse_packages(metadata)?;
    let shipping_edges = parse_shipping_edges(metadata)?;
    let workspace_root = metadata["workspace_root"].as_str().ok_or(MetadataError {
        field: "workspace_root",
    })?;
    let crates_dir = Path::new(workspace_root).join("crates");

    let roots: Vec<&str> = metadata["workspace_members"]
        .as_array()
        .ok_or(MetadataError {
            field: "workspace_members",
        })?
        .iter()
        .filter_map(Value::as_str)
        .filter(|id| {
            packages
                .get(id)
                .is_some_and(|package| Path::new(package.manifest_path).starts_with(&crates_dir))
        })
        .collect();

    let mut violations = BTreeSet::new();
    let mut reached = BTreeSet::new();
    for &root in &roots {
        // Breadth-first, remembering each package's parent so we can print the path.
        let mut parent: HashMap<&str, Option<&str>> = HashMap::from([(root, None)]);
        let mut queue = VecDeque::from([root]);
        while let Some(id) = queue.pop_front() {
            reached.insert(id);
            let Some(package) = packages.get(id) else {
                continue;
            };
            if let Some(reason) = classify(package, policy) {
                violations.insert(Violation {
                    path: path_to(id, &parent, &packages),
                    package: package.name.to_owned(),
                    reason,
                });
            }
            for &dependency in shipping_edges.get(id).map_or(&[][..], Vec::as_slice) {
                if !parent.contains_key(dependency) {
                    parent.insert(dependency, Some(id));
                    queue.push_back(dependency);
                }
            }
        }
    }

    Ok(Report {
        crates_checked: roots.len(),
        packages_reached: reached.len(),
        violations: violations.into_iter().collect(),
    })
}

/// Applies the policy rules to one package, most specific rule first.
fn classify(package: &Package<'_>, policy: &Policy) -> Option<Reason> {
    if policy.banned_crates.contains(&package.name) {
        return Some(Reason::NativeBuildTool);
    }
    if policy.allowed_sys_crates.contains(&package.name) {
        return None;
    }
    if package.name.ends_with("-sys") {
        return Some(Reason::SysCrate);
    }
    match package.links {
        Some(library) if !policy.allowed_links.contains(&package.name) => {
            Some(Reason::LinksNativeLibrary(library.to_owned()))
        }
        _ => None,
    }
}

/// Walks parent links back to the root and returns the path root-first.
fn path_to(
    id: &str,
    parent: &HashMap<&str, Option<&str>>,
    packages: &HashMap<&str, Package<'_>>,
) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = Some(id);
    while let Some(step) = current {
        path.push(
            packages
                .get(step)
                .map_or_else(|| step.to_owned(), Package::label),
        );
        current = parent.get(step).copied().flatten();
    }
    path.reverse();
    path
}

fn parse_packages(metadata: &Value) -> Result<HashMap<&str, Package<'_>>, MetadataError> {
    let list = metadata["packages"]
        .as_array()
        .ok_or(MetadataError { field: "packages" })?;
    list.iter()
        .map(|package| {
            let field =
                |name: &'static str| package[name].as_str().ok_or(MetadataError { field: name });
            Ok((
                field("id")?,
                Package {
                    name: field("name")?,
                    version: field("version")?,
                    manifest_path: field("manifest_path")?,
                    links: package["links"].as_str(),
                },
            ))
        })
        .collect()
}

/// Dependency edges that end up in a shipped build: normal and build dependencies.
///
/// Cargo lists one `dep_kinds` entry per way a dependency is used; `kind` is `null` for
/// a normal dependency, `"build"` for a build dependency and `"dev"` for a dev-dependency.
fn parse_shipping_edges(metadata: &Value) -> Result<HashMap<&str, Vec<&str>>, MetadataError> {
    let nodes = metadata["resolve"]["nodes"]
        .as_array()
        .ok_or(MetadataError {
            field: "resolve.nodes",
        })?;
    let mut edges = HashMap::new();
    for node in nodes {
        let id = node["id"].as_str().ok_or(MetadataError {
            field: "resolve.nodes[].id",
        })?;
        let dependencies = node["deps"]
            .as_array()
            .ok_or(MetadataError {
                field: "resolve.nodes[].deps",
            })?
            .iter()
            .filter(|dependency| {
                dependency["dep_kinds"].as_array().is_some_and(|kinds| {
                    kinds
                        .iter()
                        .any(|kind| kind["kind"].as_str() != Some("dev"))
                })
            })
            .filter_map(|dependency| dependency["pkg"].as_str())
            .collect();
        edges.insert(id, dependencies);
    }
    Ok(edges)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Builds a minimal `cargo metadata` document. Package IDs are `"<name> 1.0.0"`.
    #[derive(Default)]
    struct Fixture {
        packages: Vec<Value>,
        members: Vec<String>,
        edges: HashMap<String, Vec<Value>>,
    }

    impl Fixture {
        fn add(mut self, name: &str, manifest_dir: &str, links: Option<&str>) -> Self {
            let id = format!("{name} 1.0.0");
            self.packages.push(json!({
                "id": id, "name": name, "version": "1.0.0", "links": links,
                "manifest_path": format!("{manifest_dir}/{name}/Cargo.toml"),
            }));
            self.edges.entry(id).or_default();
            self
        }
        /// A shipped crate: workspace member under `crates/`.
        fn shipped(mut self, name: &str) -> Self {
            self.members.push(format!("{name} 1.0.0"));
            self.add(name, "/ws/crates", None)
        }
        /// A workspace member outside `crates/`, like `xtask`.
        fn tool(mut self, name: &str) -> Self {
            self.members.push(format!("{name} 1.0.0"));
            self.add(name, "/ws", None)
        }
        /// A package from crates.io.
        fn external(self, name: &str, links: Option<&str>) -> Self {
            self.add(name, "/registry", links)
        }
        /// `kind`: `None` for normal, `Some("build")`, or `Some("dev")`.
        fn depends(mut self, from: &str, to: &str, kind: Option<&str>) -> Self {
            self.edges
                .get_mut(&format!("{from} 1.0.0"))
                .expect("unknown package")
                .push(json!({
                    "pkg": format!("{to} 1.0.0"),
                    "dep_kinds": [{ "kind": kind, "target": null }],
                }));
            self
        }
        fn check(self) -> Report {
            let nodes: Vec<Value> = self
                .edges
                .into_iter()
                .map(|(id, deps)| json!({ "id": id, "deps": deps }))
                .collect();
            let metadata = json!({
                "packages": self.packages,
                "workspace_members": self.members,
                "workspace_root": "/ws",
                "resolve": { "nodes": nodes },
            });
            check(&metadata, &DEFAULT_POLICY).expect("fixture is well formed")
        }
    }

    #[test]
    fn pure_graph_passes() {
        let report = Fixture::default()
            .shipped("prr-a")
            .shipped("prr-b")
            .external("serde", None)
            .depends("prr-a", "prr-b", None)
            .depends("prr-b", "serde", None)
            .check();
        assert!(report.violations.is_empty());
        assert_eq!(report.crates_checked, 2);
        assert_eq!(report.packages_reached, 3);
    }

    #[test]
    fn catches_cc_as_build_dependency() {
        let report = Fixture::default()
            .shipped("prr-a")
            .external("cc", None)
            .depends("prr-a", "cc", Some("build"))
            .check();
        assert_eq!(
            report.violations,
            [Violation {
                path: vec!["prr-a 1.0.0".into(), "cc 1.0.0".into()],
                package: "cc".into(),
                reason: Reason::NativeBuildTool,
            }]
        );
    }

    #[test]
    fn catches_cc_through_a_transitive_dependency_and_prints_the_path() {
        let report = Fixture::default()
            .shipped("prr-a")
            .external("zstd", None)
            .external("cc", None)
            .depends("prr-a", "zstd", None)
            .depends("zstd", "cc", Some("build"))
            .check();
        assert_eq!(report.violations.len(), 1);
        let printed = report.violations[0].to_string();
        assert!(
            printed.contains("prr-a 1.0.0 -> zstd 1.0.0 -> cc 1.0.0"),
            "{printed}"
        );
        assert!(printed.contains("fix:"), "{printed}");
    }

    #[test]
    fn ignores_dev_dependencies() {
        let report = Fixture::default()
            .shipped("prr-a")
            .external("cc", None)
            .depends("prr-a", "cc", Some("dev"))
            .check();
        assert!(report.violations.is_empty());
    }

    #[test]
    fn ignores_crates_outside_crates_dir() {
        let report = Fixture::default()
            .tool("xtask")
            .external("cc", None)
            .depends("xtask", "cc", None)
            .check();
        assert!(report.violations.is_empty());
        assert_eq!(report.crates_checked, 0);
    }

    #[test]
    fn catches_sys_crates_but_allows_os_bindings() {
        let report = Fixture::default()
            .shipped("prr-a")
            .external("openssl-sys", Some("openssl"))
            .external("linux-raw-sys", None)
            .depends("prr-a", "openssl-sys", None)
            .depends("prr-a", "linux-raw-sys", None)
            .check();
        assert_eq!(report.violations.len(), 1);
        assert_eq!(report.violations[0].package, "openssl-sys");
        assert_eq!(report.violations[0].reason, Reason::SysCrate);
    }

    #[test]
    fn catches_links_key_on_any_package() {
        let report = Fixture::default()
            .shipped("prr-a")
            .external("ring", Some("ring_core"))
            .depends("prr-a", "ring", None)
            .check();
        assert_eq!(
            report.violations[0].reason,
            Reason::LinksNativeLibrary("ring_core".into())
        );
    }

    #[test]
    fn reports_missing_fields_instead_of_panicking() {
        let error = check(&json!({}), &DEFAULT_POLICY).unwrap_err();
        assert_eq!(error, MetadataError { field: "packages" });
    }
}
