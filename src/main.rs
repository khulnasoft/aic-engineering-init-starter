use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

static AGENT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/.agent");
static CONFIGURATION_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/configuration");
static SKILLS_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");
static UTILS_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/utils");

const ROOT_TEMPLATE_FILES: &[(&str, &str)] = &[
    (".gitignore", include_str!("../.gitignore")),
    ("AGENT.md", include_str!("../AGENT.md")),
    ("CHANGELOG.md", include_str!("../CHANGELOG.md")),
    ("CONTRIBUTING.md", include_str!("../CONTRIBUTING.md")),
    ("LICENSE", include_str!("../LICENSE")),
    ("README.md", include_str!("../README.md")),
    ("SECURITY.md", include_str!("../SECURITY.md")),
    (
        "TEMPLATE-MANIFEST.yaml",
        include_str!("../TEMPLATE-MANIFEST.yaml"),
    ),
    (
        ".github/workflows/template.yml",
        include_str!("../.github/workflows/template.yml"),
    ),
];

#[derive(Debug, Parser)]
#[command(name = "aic", version, about = "AI-native engineering control plane")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Bootstrap AIC project files in the current repository.
    Init,
    /// Display and validate project configuration.
    Config,
    /// Detect the repository technology stack.
    Stack,
    /// Summarize repository and AIC state.
    Inspect,
    /// Display persisted engineering state.
    Status,
    /// Check project configuration and tooling health.
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => init(&std::env::current_dir()?),
        Command::Config => config(&find_root()?),
        Command::Stack => stack(&find_root()?),
        Command::Inspect => inspect(&find_root()?),
        Command::Status => status(&find_root()?),
        Command::Doctor => doctor(&find_root()?),
    }
}

fn init(root: &Path) -> Result<()> {
    for (path, contents) in ROOT_TEMPLATE_FILES {
        materialize_file(root, Path::new(path), contents.as_bytes())?;
    }
    materialize_directory(&AGENT_TEMPLATE, root, Path::new(".agent"))?;
    materialize_directory(&CONFIGURATION_TEMPLATE, root, Path::new("configuration"))?;
    materialize_directory(&SKILLS_TEMPLATE, root, Path::new("skills"))?;
    materialize_directory(&UTILS_TEMPLATE, root, Path::new("utils"))?;
    Ok(())
}

fn materialize_directory(directory: &Dir<'_>, root: &Path, destination: &Path) -> Result<()> {
    for file in directory.files() {
        materialize_file(root, &destination.join(file.path()), file.contents())?;
    }

    for child in directory.dirs() {
        materialize_directory(child, root, destination)?;
    }

    Ok(())
}

fn materialize_file(root: &Path, relative_path: &Path, contents: &[u8]) -> Result<()> {
    if relative_path
        .components()
        .any(|component| component.as_os_str() == ".DS_Store")
    {
        return Ok(());
    }
    let target = root.join(relative_path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create {}", display_path(parent).display()))?;
    }
    if target.exists() {
        println!("skip {} (already exists)", display_path(&target).display());
        return Ok(());
    }
    fs::write(&target, contents)
        .with_context(|| format!("write {}", display_path(&target).display()))?;
    println!("create {}", display_path(&target).display());
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ProjectConfig {
    version: u32,
    project: ProjectDetails,
    #[serde(default)]
    stack: Option<StackDetails>,
    #[serde(default)]
    runtime: Option<RuntimeDetails>,
}

#[derive(Debug, Deserialize)]
struct ProjectDetails {
    name: String,
    #[serde(rename = "type")]
    project_type: String,
}

#[derive(Debug, Deserialize)]
struct StackDetails {
    language: String,
    runtime: String,
    framework: String,
    package_manager: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeDetails {
    declared: bool,
    value: Option<String>,
}

fn find_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join(".agent/project.yaml").is_file() {
            return Ok(current);
        }
        if !current.pop() {
            anyhow::bail!("no AIC project found; run `aic init` first");
        }
    }
}

fn load_config(root: &Path) -> Result<ProjectConfig> {
    let path = root.join(".agent/project.yaml");
    let contents = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let config: ProjectConfig =
        serde_yaml::from_str(&contents).with_context(|| format!("parse {}", path.display()))?;
    if config.version != 1 {
        anyhow::bail!("unsupported project schema version {}", config.version);
    }
    if config.project.project_type.trim().is_empty() {
        anyhow::bail!("project type must not be empty");
    }
    if let Some(runtime) = &config.runtime {
        if runtime.declared
            && runtime
                .value
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty()
        {
            anyhow::bail!("declared runtime must have a value");
        }
    }
    Ok(config)
}

fn config(root: &Path) -> Result<()> {
    let config = load_config(root)?;
    println!(
        "project: {}",
        if config.project.name.trim().is_empty() {
            "(unnamed)"
        } else {
            &config.project.name
        }
    );
    println!("type: {}", config.project.project_type);
    if let Some(stack) = config.stack {
        println!("language: {}", stack.language);
        println!("runtime: {}", stack.runtime);
        println!("framework: {}", stack.framework);
        println!("package_manager: {}", stack.package_manager);
    } else {
        println!("stack: undeclared");
    }
    Ok(())
}

fn stack(root: &Path) -> Result<()> {
    let detected = detect_stack(root);
    let contents = format!(
        "version: 1\n\nstack:\n  language: {}\n  runtime: {}\n  framework: {}\n  package_manager: {}\n",
        detected.language, detected.runtime, detected.framework, detected.package_manager
    );
    fs::write(root.join(".agent/stack.yaml"), contents).context("write detected stack")?;
    println!("language: {}", detected.language);
    println!("runtime: {}", detected.runtime);
    println!("framework: {}", detected.framework);
    println!("package_manager: {}", detected.package_manager);
    Ok(())
}

fn inspect(root: &Path) -> Result<()> {
    let config = load_config(root)?;
    println!(
        "project: {}",
        if config.project.name.trim().is_empty() {
            "(unnamed)"
        } else {
            &config.project.name
        }
    );
    println!("aic_root: {}", root.display());
    println!("tracked_state: {}", root.join(".agent").display());
    println!("configuration: valid");
    println!("git_repository: {}", root.join(".git").exists());
    Ok(())
}

fn status(root: &Path) -> Result<()> {
    load_config(root)?;
    println!("desired: configured");
    println!("planned: not_started");
    println!("actual: detected");
    println!("verified: evidence_required");
    Ok(())
}

fn doctor(root: &Path) -> Result<()> {
    let mut failures = Vec::new();
    if let Err(error) = load_config(root) {
        failures.push(error.to_string());
    }
    for path in [
        ".agent/project.yaml",
        ".agent/state.yaml",
        "TEMPLATE-MANIFEST.yaml",
        "AGENT.md",
    ] {
        if !root.join(path).is_file() {
            failures.push(format!("missing {path}"));
        }
    }
    if failures.is_empty() {
        println!("doctor: healthy");
        Ok(())
    } else {
        for failure in failures {
            eprintln!("doctor: {failure}");
        }
        anyhow::bail!("project health checks failed")
    }
}

#[derive(Debug)]
struct DetectedStack {
    language: &'static str,
    runtime: &'static str,
    framework: &'static str,
    package_manager: &'static str,
}

fn detect_stack(root: &Path) -> DetectedStack {
    if root.join("Cargo.toml").is_file() {
        return DetectedStack {
            language: "rust",
            runtime: "rust",
            framework: "none",
            package_manager: "cargo",
        };
    }
    if root.join("package.json").is_file() {
        let package_manager = if root.join("pnpm-lock.yaml").is_file() {
            "pnpm"
        } else if root.join("yarn.lock").is_file() {
            "yarn"
        } else {
            "npm"
        };
        return DetectedStack {
            language: "typescript_or_javascript",
            runtime: "node",
            framework: "unknown",
            package_manager,
        };
    }
    DetectedStack {
        language: "unknown",
        runtime: "unknown",
        framework: "unknown",
        package_manager: "unknown",
    }
}

fn display_path(path: &Path) -> PathBuf {
    path.strip_prefix(std::env::current_dir().unwrap_or_default())
        .unwrap_or(path)
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn init_creates_templates_without_overwriting_existing_files() {
        let directory = tempdir().expect("temporary directory");
        let existing = directory.path().join(".agent/project.yaml");
        fs::create_dir_all(existing.parent().expect("parent")).expect("agent directory");
        fs::write(&existing, "custom: true\n").expect("existing config");

        init(directory.path()).expect("initialize project");

        assert_eq!(
            fs::read_to_string(existing).expect("read config"),
            "custom: true\n"
        );
        assert!(directory.path().join(".agent/project.yaml").exists());
        assert!(directory.path().join(".agent/state.yaml").exists());
        assert!(directory
            .path()
            .join(".agent/decisions/ADR-001-work-item-identity.yaml")
            .exists());
        assert!(directory
            .path()
            .join("configuration/workflow.yaml")
            .exists());
        assert!(directory.path().join("skills/coding/SKILL.md").exists());
        assert!(directory
            .path()
            .join(".github/workflows/template.yml")
            .exists());
        assert!(!directory.path().join("Cargo.toml").exists());
        assert!(!directory.path().join("src/main.rs").exists());
        assert!(!directory.path().join("docs").exists());
        assert!(!directory.path().join("examples").exists());
        assert!(!directory.path().join("schemas").exists());
        assert!(!directory.path().join("target").exists());

        let generated_agent = fs::read_to_string(directory.path().join("AGENT.md"))
            .expect("generated agent contract");
        init(directory.path()).expect("initialize project again");
        assert_eq!(
            fs::read_to_string(directory.path().join("AGENT.md"))
                .expect("generated agent contract after second init"),
            generated_agent
        );
    }

    #[test]
    fn detects_rust_stack_from_cargo_manifest() {
        let directory = tempdir().expect("temporary directory");
        fs::write(
            directory.path().join("Cargo.toml"),
            "[package]\nname = \"fixture\"\n",
        )
        .expect("cargo manifest");

        let detected = detect_stack(directory.path());

        assert_eq!(detected.language, "rust");
        assert_eq!(detected.package_manager, "cargo");
    }
}
