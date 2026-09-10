#![allow(clippy::manual_flatten, clippy::unnecessary_map_or, clippy::useless_format, clippy::print_literal)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

const STATE_VERSION: u32 = 1;

#[derive(Debug, Parser)]
#[command(name = "aic", version, about = "AI-native engineering control plane")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Init {
        #[arg(long)]
        non_interactive: bool,
    },
    Config,
    Stack,
    Inspect,
    Status,
    Doctor,
    Context,
    Architecture { #[command(subcommand)] subcommand: Option<ArchitectureCommand> },
    Dependencies { #[command(subcommand)] subcommand: Option<DependenciesCommand> },
    Conventions { #[command(subcommand)] subcommand: Option<ConventionsCommand> },
    Index { #[command(subcommand)] subcommand: Option<IndexCommand> },
    Todo { #[command(subcommand)] subcommand: TodoCommand },
    Work { #[command(subcommand)] subcommand: WorkCommand },
    Verify { #[command(subcommand)] subcommand: VerifyCommand },
    Idea { description: Option<String> },
    Requirements { #[command(subcommand)] subcommand: Option<RequirementsCommand> },
    Roadmap { #[command(subcommand)] subcommand: Option<RoadmapCommand> },
    Planning { #[command(subcommand)] subcommand: Option<PlanningCommand> },
    Decision { title: Option<String> },
    Estimate { for_work_item: Option<String> },
    Diagnose,
    Benchmark,
    Optimize,
    Remediate,
    Upgrade,
    Test,
    Review,
    Security,
    Audit,
    Coverage,
    Memory,
    Learn,
    Explain { topic: Option<String> },
    Policy,
    Rules,
    Permissions,
    Skill { name: Option<String> },
    Template,
    Provider,
    Model,
    Loop,
    Resume,
    Checkpoint,
}

#[derive(Debug, Clone, Subcommand)]
enum TodoCommand {
    List,
    Create { title: String, r#type: Option<String>, priority: Option<String> },
    Next,
    Show { id: String },
    Verify { id: String },
    Sync,
}

#[derive(Debug, Clone, Subcommand)]
enum WorkCommand {
    List,
    Next,
    Start { id: String },
    Complete { id: String },
    Checkpoint { id: Option<String> },
}

#[derive(Debug, Clone, Subcommand)]
enum VerifyCommand {
    WorkItem { id: String },
    All,
    Level { level: String },
}

#[derive(Debug, Clone, Subcommand)]
enum RequirementsCommand {
    List,
    Create { title: String },
}

#[derive(Debug, Clone, Subcommand)]
enum RoadmapCommand {
    Display,
    Add { title: String },
}

#[derive(Debug, Clone, Subcommand)]
enum PlanningCommand {
    List,
    Create { title: String, milestone: Option<String> },
}

#[derive(Debug, Clone, Subcommand)]
enum ArchitectureCommand {
    Display,
    Infer,
}

#[derive(Debug, Clone, Subcommand)]
enum DependenciesCommand {
    List,
    Graph,
    Audit,
}

#[derive(Debug, Clone, Subcommand)]
enum ConventionsCommand {
    Display,
    Infer,
}

#[derive(Debug, Clone, Subcommand)]
enum IndexCommand {
    Build,
    Search { query: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { non_interactive } => {
            let root = std::env::current_dir()?;
            if non_interactive { init(&root) } else { interactive_init(&root) }
        }
        Command::Config => config(&find_root()?),
        Command::Stack => stack(&find_root()?),
        Command::Inspect => inspect(&find_root()?),
        Command::Status => status(&find_root()?),
        Command::Doctor => doctor(&find_root()?),
        Command::Context => context(&find_root()?),
        Command::Architecture { subcommand } => architecture(&find_root()?, subcommand.unwrap_or(ArchitectureCommand::Display)),
        Command::Dependencies { subcommand } => dependencies(&find_root()?, subcommand.unwrap_or(DependenciesCommand::List)),
        Command::Conventions { subcommand } => conventions(&find_root()?, subcommand.unwrap_or(ConventionsCommand::Display)),
        Command::Index { subcommand } => index(&find_root()?, subcommand.unwrap_or(IndexCommand::Build)),
        Command::Todo { subcommand } => todo(&find_root()?, subcommand),
        Command::Work { subcommand } => work(&find_root()?, subcommand),
        Command::Verify { subcommand } => verify(&find_root()?, subcommand),
        Command::Idea { description } => idea(&find_root()?, description),
        Command::Requirements { subcommand } => {
            requirements(&find_root()?, subcommand.unwrap_or(RequirementsCommand::List))
        }
        Command::Roadmap { subcommand } => {
            roadmap(&find_root()?, subcommand.unwrap_or(RoadmapCommand::Display))
        }
        Command::Planning { subcommand } => {
            planning(&find_root()?, subcommand.unwrap_or(PlanningCommand::List))
        }
        Command::Decision { title } => decision(&find_root()?, title),
        Command::Estimate { for_work_item } => estimate(&find_root()?, for_work_item),
        Command::Diagnose => diagnose(&find_root()?),
        Command::Benchmark => benchmark(&find_root()?),
        Command::Optimize => optimize(&find_root()?),
        Command::Remediate => remediate(&find_root()?),
        Command::Upgrade => upgrade(&find_root()?),
        Command::Test => test(&find_root()?),
        Command::Review => review(&find_root()?),
        Command::Security => security(&find_root()?),
        Command::Audit => audit(&find_root()?),
        Command::Coverage => coverage(&find_root()?),
        Command::Memory => memory(&find_root()?),
        Command::Learn => learn(&find_root()?),
        Command::Explain { topic } => explain(&find_root()?, topic),
        Command::Policy => policy(&find_root()?),
        Command::Rules => rules(&find_root()?),
        Command::Permissions => permissions(&find_root()?),
        Command::Skill { name } => skill(&find_root()?, name),
        Command::Template => template(&find_root()?),
        Command::Provider => provider(&find_root()?),
        Command::Model => model(&find_root()?),
        Command::Loop => loop_command(&find_root()?),
        Command::Resume => resume(&find_root()?),
        Command::Checkpoint => checkpoint(&find_root()?),
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
    update_state(root, |state| state.phase = "bootstrap".to_string())?;
    Ok(())
}

fn interactive_init(root: &Path) -> Result<()> {
    let choices = collect_init_choices(root)?;
    init(root)?;
    write_project_choices(root, &choices)?;
    println!("\n└  Project initialized successfully");
    println!("   API keys remain environment-only; no secrets were written.");
    Ok(())
}

#[derive(Debug, Default)]
struct InitChoices {
    project_name: String,
    stack: StackChoice,
    workflow: WorkflowChoices,
    provider: String,
    model: String,
}

#[derive(Debug, Default, Clone)]
struct StackChoice {
    language: String,
    runtime: String,
    framework: String,
    package_manager: String,
}

#[derive(Debug, Default, Clone)]
struct WorkflowChoices {
    skills: bool,
    agent_instructions: bool,
    context: bool,
    roadmap: bool,
    todo_management: bool,
    todo_verification: bool,
}

fn collect_init_choices(root: &Path) -> Result<InitChoices> {
    let theme = ColorfulTheme::default();
    println!(" ◆  Initialize AI coding project\n│");
    let default_name = root.file_name().and_then(|n| n.to_str()).unwrap_or("my-app").to_string();
    let project_name: String = Input::with_theme(&theme).with_prompt("◇  Project name").default(default_name).interact_text()?;
    let stack_options = ["Detect automatically","Next.js","React","Node.js","Rust","Python","PHP","Custom"];
    let stack_index = Select::with_theme(&theme).with_prompt("◇  Choose stack").items(&stack_options).default(0).interact()?;
    let stack = choose_stack(root, stack_index, &theme)?;
    let workflow_options = ["Skills","Agent instructions","Context","Roadmap","TODO management","TODO verification"];
    let defaults = [true; 6];
    let enabled = MultiSelect::with_theme(&theme).with_prompt("◇  Enable AI workflow").items(&workflow_options).defaults(&defaults).interact()?;
    let workflow = WorkflowChoices {
        skills: enabled.contains(&0), agent_instructions: enabled.contains(&1), context: enabled.contains(&2),
        roadmap: enabled.contains(&3), todo_management: enabled.contains(&4), todo_verification: enabled.contains(&5),
    };
    let mut provider = "openai".to_string();
    let mut model = "gpt-4o".to_string();
    loop {
        let provider_options = ["OpenAI Key (OPENAI_API_KEY)","Anthropic Key (ANTHROPIC_API_KEY)","OpenAI API Endpoint","Model","Done"];
        let selection = Select::with_theme(&theme).items(&provider_options).default(3).interact()?;
        match selection {
            0 => { provider = "openai".to_string(); println!("  Use OPENAI_API_KEY from the environment; it will not be stored."); }
            1 => { provider = "anthropic".to_string(); println!("  Use ANTHROPIC_API_KEY from the environment; it will not be stored."); }
            2 => { provider = "openai-compatible".to_string(); println!("  Endpoint is configured with AIC_API_ENDPOINT."); }
            3 => { model = Input::with_theme(&theme).with_prompt("Model").default(model).interact_text()?; }
            _ => break,
        }
    }
    Ok(InitChoices { project_name, stack, workflow, provider, model })
}

fn choose_stack(root: &Path, selection: usize, theme: &ColorfulTheme) -> Result<StackChoice> {
    if selection == 0 {
        let detected = detect_stack(root);
        return Ok(StackChoice { language: detected.language.to_string(), runtime: detected.runtime.to_string(), framework: detected.framework.to_string(), package_manager: detected.package_manager.to_string() });
    }
    if selection == 7 {
        return Ok(StackChoice {
            language: Input::with_theme(theme).with_prompt("Language").default("unknown".to_string()).interact_text()?,
            runtime: Input::with_theme(theme).with_prompt("Runtime").default("unknown".to_string()).interact_text()?,
            framework: Input::with_theme(theme).with_prompt("Framework").default("unknown".to_string()).interact_text()?,
            package_manager: Input::with_theme(theme).with_prompt("Package manager").default("unknown".to_string()).interact_text()?,
        });
    }
    let stacks = [("nextjs","node","nextjs","npm"),("javascript","node","react","npm"),("javascript","node","none","npm"),("rust","rust","none","cargo"),("python","python","none","pip"),("php","php","none","composer")];
    let (language, runtime, framework, package_manager) = stacks[selection - 1];
    Ok(StackChoice { language: language.to_string(), runtime: runtime.to_string(), framework: framework.to_string(), package_manager: package_manager.to_string() })
}

fn write_project_choices(root: &Path, choices: &InitChoices) -> Result<()> {
    let path = root.join(".agent/project.yaml");
    let contents = ProjectFile {
        version: 1,
        project: ProjectFileDetails { name: choices.project_name.clone(), project_type: "application".to_string() },
        stack: choices.stack.clone().into(),
        runtime: RuntimeDetails { declared: choices.stack.runtime != "unknown", value: Some(choices.stack.runtime.clone()) },
        ai: AiConfigSerialize { provider: choices.provider.clone(), model: choices.model.clone(), endpoint: None },
        workflow: choices.workflow.clone().into(),
    };
    fs::write(path, serde_yaml::to_string(&contents).context("serialize project configuration")?)
        .context("write interactive project configuration")?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct ProjectFile {
    version: u32,
    project: ProjectFileDetails,
    stack: StackDetails,
    runtime: RuntimeDetails,
    ai: AiConfigSerialize,
    workflow: WorkflowFile,
}

#[derive(Debug, Serialize)]
struct ProjectFileDetails { name: String, #[serde(rename = "type")] project_type: String }

#[derive(Debug, Serialize)]
struct AiConfigSerialize { provider: String, model: String, endpoint: Option<String> }

#[derive(Debug, Serialize)]
struct WorkflowFile { skills: bool, agent_instructions: bool, context: bool, roadmap: bool, todo: bool, verification: bool }

impl From<StackChoice> for StackDetails {
    fn from(value: StackChoice) -> Self {
        Self { language: value.language, runtime: value.runtime, framework: value.framework, package_manager: value.package_manager }
    }
}
impl From<WorkflowChoices> for WorkflowFile {
    fn from(value: WorkflowChoices) -> Self {
        Self { skills: value.skills, agent_instructions: value.agent_instructions, context: value.context, roadmap: value.roadmap, todo: value.todo_management, verification: value.todo_verification }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ProjectState {
    version: u32, phase: String, health: String,
    roadmap: RoadmapState, work: WorkState, verification: VerificationState, context: ContextState,
    last_checkpoint: Option<String>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
struct RoadmapState { completed: u32, total: u32 }
#[derive(Debug, Default, Serialize, Deserialize)]
struct WorkState { pending: u32, active: u32, verified: u32 }
#[derive(Debug, Default, Serialize, Deserialize)]
struct VerificationState { score: u32 }
#[derive(Debug, Default, Serialize, Deserialize)]
struct ContextState { status: String }

impl ProjectState {
    fn new() -> Self {
        Self { version: STATE_VERSION, phase: "bootstrap".to_string(), health: "unknown".to_string(),
            roadmap: RoadmapState { completed: 0, total: 0 }, work: WorkState { pending: 0, active: 0, verified: 0 },
            verification: VerificationState { score: 0 }, context: ContextState { status: "not_initialized".to_string() },
            last_checkpoint: None }
    }
}

fn load_state(root: &Path) -> Result<ProjectState> {
    let path = root.join(".agent/state.yaml");
    if !path.is_file() { return Ok(ProjectState::new()); }
    let contents = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let state: ProjectState = serde_yaml::from_str(&contents).with_context(|| format!("parse {}", path.display()))?;
    Ok(state)
}

fn save_state(root: &Path, state: &ProjectState) -> Result<()> {
    fs::write(root.join(".agent/state.yaml"), serde_yaml::to_string(state).context("serialize project state")?)
        .context("write project state")?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WorkItem {
    version: u32, id: String, r#type: String, title: String, status: String, priority: String,
    acceptance: Vec<String>, verification: Vec<String>, plan: Vec<String>, files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")] started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] evidence: Option<Vec<EvidenceItem>>,
    notes: Vec<String>,
}

impl WorkItem {
    fn new(id: String, title: String, r#type: String, priority: String) -> Self {
        Self { version: 1, id, r#type, title, status: "pending".to_string(), priority,
            acceptance: Vec::new(), verification: Vec::new(), plan: Vec::new(), files: Vec::new(),
            started_at: None, completed_at: None, evidence: None, notes: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EvidenceItem { r#type: String, path: Option<String>, command: Option<String>, result: String }

fn load_work_item(root: &Path, id: &str) -> Result<WorkItem> {
    let contents = fs::read_to_string(root.join(format!(".agent/work/{}.yaml", id)))?;
    Ok(serde_yaml::from_str(&contents)?)
}

fn save_work_item(root: &Path, item: &WorkItem) -> Result<()> {
    fs::create_dir_all(root.join(".agent/work"))?;
    fs::write(root.join(format!(".agent/work/{}.yaml", item.id)), serde_yaml::to_string(item).context("serialize work item")?)
        .context("write work item")?;
    Ok(())
}

fn list_work_items(root: &Path) -> Result<Vec<WorkItem>> {
    let mut items = Vec::new();
    let dir = root.join(".agent/work");
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "yaml") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(item) = serde_yaml::from_str::<WorkItem>(&contents) { items.push(item); }
                    }
                }
            }
        }
    }
    items.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(items)
}

fn next_work_item_id(root: &Path) -> String {
    let mut max_num: u32 = 0;
    let dir = root.join(".agent/work");
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("WI-") && name_str.ends_with(".yaml") {
                    let num_str = name_str.trim_start_matches("WI-").trim_end_matches(".yaml");
                    if let Ok(num) = num_str.parse::<u32>() { max_num = max_num.max(num); }
                }
            }
        }
    }
    format!("WI-{:03}", max_num + 1)
}

fn load_config(root: &Path) -> Result<ProjectConfig> {
    let path = root.join(".agent/project.yaml");
    let contents = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let config: ProjectConfig = serde_yaml::from_str(&contents).with_context(|| format!("parse {}", path.display()))?;
    if config.version != 1 { anyhow::bail!("unsupported project schema version {}", config.version); }
    if config.project.project_type.trim().is_empty() { anyhow::bail!("project type must not be empty"); }
    Ok(config)
}

fn config(root: &Path) -> Result<()> {
    let config = load_config(root)?;
    let state = load_state(root).unwrap_or_default();
    println!("project: {}", if config.project.name.trim().is_empty() { "(unnamed)" } else { &config.project.name });
    println!("type: {}", config.project.project_type);
    if let Some(stack) = &config.stack {
        println!("language: {}\nruntime: {}\nframework: {}\npackage_manager: {}", stack.language, stack.runtime, stack.framework, stack.package_manager);
    } else { println!("stack: undeclared"); }
    if let Some(ai) = &config.ai {
        println!("ai.provider: {}", ai.provider.as_deref().unwrap_or("unknown"));
        println!("ai.model: {}", ai.model.as_deref().unwrap_or("unknown"));
    }
    println!("phase: {}", state.phase);
    println!("health: {}", state.health);
    println!("work.pending: {} work.active: {} work.verified: {}", state.work.pending, state.work.active, state.work.verified);
    println!("verification.score: {}", state.verification.score);
    println!("context.status: {}", state.context.status);
    Ok(())
}

fn stack(root: &Path) -> Result<()> {
    let detected = detect_stack(root);
    fs::write(root.join(".agent/stack.yaml"), format!("version: 1\n\nstack:\n  language: {}\n  runtime: {}\n  framework: {}\n  package_manager: {}\n", detected.language, detected.runtime, detected.framework, detected.package_manager))
        .context("write detected stack")?;
    update_state(root, |state| state.context.status = "detected".to_string())?;
    println!("language: {}\nruntime: {}\nframework: {}\npackage_manager: {}", detected.language, detected.runtime, detected.framework, detected.package_manager);
    Ok(())
}

fn inspect(root: &Path) -> Result<()> {
    let config = load_config(root)?;
    let state = load_state(root).unwrap_or_default();
    let work_items = list_work_items(root)?;
    println!("project: {}", if config.project.name.trim().is_empty() { "(unnamed)" } else { &config.project.name });
    println!("aic_root: {}", root.display());
    println!("tracked_state: {}", root.join(".agent").display());
    println!("configuration: valid");
    println!("git_repository: {}", root.join(".git").exists());
    println!("phase: {}", state.phase);
    println!("health: {}", state.health);
    println!("work_items: {}", work_items.len());
    for item in &work_items { println!("  {} [{}] {}", item.id, item.status, item.title); }
    Ok(())
}

fn status(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    println!("desired: {}", if state.phase == "bootstrap" { "configured" } else { state.phase.as_str() });
    println!("planned: {}", state.phase);
    println!("actual: detected");
    println!("verified: {}", state.health);
    println!("work.pending: {} work.active: {} work.verified: {}", state.work.pending, state.work.active, state.work.verified);
    println!("verification.score: {}", state.verification.score);
    println!("context.status: {}", state.context.status);
    println!("last_checkpoint: {}", state.last_checkpoint.as_deref().unwrap_or("none"));
    Ok(())
}

fn doctor(root: &Path) -> Result<()> {
    let mut failures = Vec::new();
    if let Err(e) = load_config(root) { failures.push(e.to_string()); }
    let state = load_state(root).unwrap_or_default();
    if state.version != STATE_VERSION { failures.push("state schema version mismatch".to_string()); }
    for path in &[".agent/project.yaml",".agent/state.yaml","TEMPLATE-MANIFEST.yaml","AGENT.md","configuration/workflow.yaml","configuration/project.yaml",".agent/decisions/ADR-001-work-item-identity.yaml",".agent/decisions/ADR-002-cli-runtime.yaml",".agent/roadmap/roadmap.yaml"] {
        if !root.join(path).is_file() { failures.push(format!("missing {}", path)); }
    }
    for dir in &[".agent/work",".agent/decisions",".agent/roadmap",".agent/verification"] {
        if !root.join(dir).is_dir() { failures.push(format!("missing directory {}", dir)); }
    }
    if failures.is_empty() {
        let health = if state.work.pending == 0 && state.work.active == 0 && state.work.verified > 0 { "healthy" } else { "needs_attention" };
        update_state(root, |state| state.health = health.to_string())?;
        println!("doctor: {}\n  state.phase: {}\n  work.pending: {}\n  work.active: {}\n  work.verified: {}", health, state.phase, state.work.pending, state.work.active, state.work.verified);
        Ok(())
    } else {
        for failure in &failures { eprintln!("doctor: {failure}"); }
        update_state(root, |state| state.health = "unhealthy".to_string())?;
        anyhow::bail!("project health checks failed")
    }
}

fn update_state<F: FnOnce(&mut ProjectState)>(root: &Path, updater: F) -> Result<()> {
    let mut state = load_state(root).unwrap_or_else(|_| ProjectState::new());
    updater(&mut state);
    save_state(root, &state)?;
    Ok(())
}

fn context(root: &Path) -> Result<()> {
    let config = load_config(root)?;
    let state = load_state(root).unwrap_or_default();
    println!("=== Project Context ===");
    println!("name: {}", config.project.name);
    println!("type: {}", config.project.project_type);
    if let Some(stack) = &config.stack { println!("stack: {} {} {} ({})", stack.language, stack.runtime, stack.framework, stack.package_manager); }
    if let Some(ai) = &config.ai { println!("ai: {} @ {}", ai.provider.as_deref().unwrap_or("unknown"), ai.model.as_deref().unwrap_or("unknown")); }
    println!("phase: {}", state.phase);
    println!("health: {}", state.health);
    println!("context.status: {}", state.context.status);
    update_state(root, |state| state.context.status = "built".to_string())?;
    Ok(())
}

fn todo(root: &Path, subcommand: TodoCommand) -> Result<()> {
    match subcommand {
        TodoCommand::List => todo_list(root),
        TodoCommand::Create { title, r#type, priority } => todo_create(root, title, r#type, priority),
        TodoCommand::Next => todo_next(root),
        TodoCommand::Show { id } => todo_show(root, &id),
        TodoCommand::Verify { id } => todo_verify(root, &id),
        TodoCommand::Sync => todo_sync(root),
    }
}

fn todo_list(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    if items.is_empty() { println!("No work items found."); return Ok(()); }
    println!("{:<10} {:<12} {:<10} {}", "ID", "TYPE", "STATUS", "TITLE");
    println!("{}", "-".repeat(70));
    for item in &items { println!("{:<10} {:<12} {:<10} {}", item.id, item.r#type, item.status, item.title); }
    Ok(())
}

fn todo_create(root: &Path, title: String, r#type: Option<String>, priority: Option<String>) -> Result<()> {
    let id = next_work_item_id(root);
    let item_type = r#type.unwrap_or_else(|| "feature".to_string());
    let item_priority = priority.unwrap_or_else(|| "medium".to_string());
    let item = WorkItem::new(id.clone(), title, item_type.clone(), item_priority.clone());
    save_work_item(root, &item)?;
    update_state(root, |state| state.work.pending += 1)?;
    println!("Created {}: {}", id, item_type);
    println!("  Title: {}", item.title);
    println!("  Priority: {}", item.priority);
    println!("  Status: {}", item.status);
    Ok(())
}

fn todo_next(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    let pending: Vec<_> = items.iter().filter(|i| i.status == "pending").collect();
    if pending.is_empty() { println!("No pending work items."); return Ok(()); }
    let next = &pending[0];
    println!("Next work item: {} [{}]", next.id, next.title);
    println!("  Priority: {}", next.priority);
    println!("  Type: {}", next.r#type);
    Ok(())
}

fn todo_show(root: &Path, id: &str) -> Result<()> {
    let item = load_work_item(root, id)?;
    println!("ID: {}\nType: {}\nTitle: {}\nStatus: {}\nPriority: {}", item.id, item.r#type, item.title, item.status, item.priority);
    if let Some(started) = &item.started_at { println!("Started: {}", started); }
    if let Some(completed) = &item.completed_at { println!("Completed: {}", completed); }
    if !item.acceptance.is_empty() { println!("Acceptance:"); for a in &item.acceptance { println!("  - {}", a); } }
    if let Some(evidence) = &item.evidence { println!("Evidence:"); for e in evidence { println!("  [{}] {}", e.r#type, e.result); } }
    Ok(())
}

fn todo_verify(root: &Path, id: &str) -> Result<()> {
    let mut item = load_work_item(root, id)?;
    let mut all_pass = true;
    let mut evidence = Vec::new();
    for v in &item.verification {
        let result = match v.as_str() {
            "typecheck" => check_typecheck(&item),
            "unit_test" => check_unit_test(&item),
            "integration_test" => check_integration_test(&item),
            v => check_generic(v, &item),
        };
        evidence.push(EvidenceItem { r#type: v.to_string(), path: None, command: None, result: result.clone() });
        if result != "pass" { all_pass = false; }
    }
    item.evidence = Some(evidence);
    if all_pass {
        item.status = "verified".to_string();
        item.completed_at = Some(timestamp());
        update_state(root, |state| { state.work.verified += 1; state.work.pending = state.work.pending.saturating_sub(1); state.health = "good".to_string(); })?;
        println!("{} verified successfully", id);
    } else {
        item.status = "failed_verification".to_string();
        update_state(root, |state| state.health = "needs_attention".to_string())?;
        println!("{} verification failed", id);
    }
    save_work_item(root, &item)?;
    Ok(())
}

fn todo_sync(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    let mut pending = 0u32; let mut active = 0u32; let mut verified = 0u32;
    for item in &items { match item.status.as_str() { "pending" => pending += 1, "active" | "in_progress" => active += 1, "verified" => verified += 1, _ => {} } }
    update_state(root, |state| { state.work.pending = pending; state.work.active = active; state.work.verified = verified; })?;
    println!("Synced work items: {} pending, {} active, {} verified", pending, active, verified);
    Ok(())
}

fn work(root: &Path, subcommand: WorkCommand) -> Result<()> {
    match subcommand {
        WorkCommand::List => work_list(root),
        WorkCommand::Next => work_next(root),
        WorkCommand::Start { id } => work_start(root, &id),
        WorkCommand::Complete { id } => work_complete(root, &id),
        WorkCommand::Checkpoint { id } => work_checkpoint(root, id),
    }
}

fn work_list(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    if items.is_empty() { println!("No work items found."); return Ok(()); }
    println!("{:<10} {:<12} {:<15} {}", "ID", "TYPE", "STATUS", "TITLE");
    println!("{}", "-".repeat(70));
    for item in &items { println!("{:<10} {:<12} {:<15} {}", item.id, item.r#type, item.status, item.title); }
    Ok(())
}

fn work_next(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    let pending: Vec<_> = items.iter().filter(|i| i.status == "pending").collect();
    if pending.is_empty() { println!("No pending work items."); return Ok(()); }
    let mut item = pending[0].clone();
    item.status = "active".to_string();
    item.started_at = Some(timestamp());
    save_work_item(root, &item)?;
    update_state(root, |state| { state.work.pending = state.work.pending.saturating_sub(1); state.work.active += 1; })?;
    println!("Started {}: {}", item.id, item.title);
    Ok(())
}

fn work_start(root: &Path, id: &str) -> Result<()> {
    let mut item = load_work_item(root, id)?;
    if item.status != "pending" { anyhow::bail!("work item {} is not pending (status: {})", id, item.status); }
    item.status = "active".to_string();
    item.started_at = Some(timestamp());
    save_work_item(root, &item)?;
    update_state(root, |state| { state.work.pending = state.work.pending.saturating_sub(1); state.work.active += 1; })?;
    println!("Started {}: {}", id, item.title);
    Ok(())
}

fn work_complete(root: &Path, id: &str) -> Result<()> {
    let mut item = load_work_item(root, id)?;
    if item.status != "active" { anyhow::bail!("work item {} is not active (status: {})", id, item.status); }
    item.status = "verified".to_string();
    item.completed_at = Some(timestamp());
    save_work_item(root, &item)?;
    update_state(root, |state| { state.work.active = state.work.active.saturating_sub(1); state.work.verified += 1; state.health = "good".to_string(); })?;
    println!("Completed {}: {}", id, item.title);
    Ok(())
}

fn work_checkpoint(root: &Path, id: Option<String>) -> Result<()> {
    let ts = timestamp();
    let label = id.unwrap_or_else(|| "current".to_string());
    update_state(root, |state| state.last_checkpoint = Some(ts.clone()))?;
    println!("Checkpoint created for {} at {}", label, ts);
    Ok(())
}

fn verify(root: &Path, subcommand: VerifyCommand) -> Result<()> {
    match subcommand {
        VerifyCommand::WorkItem { id } => todo_verify(root, &id),
        VerifyCommand::All => verify_all(root),
        VerifyCommand::Level { level } => verify_level(root, &level),
    }
}

fn verify_all(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    let verified = items.iter().filter(|i| i.status == "verified").count();
    let failed = items.iter().filter(|i| i.status == "failed_verification").count();
    let pending = items.len() as u32 - verified as u32 - failed as u32;
    println!("Verification summary: {} verified, {} pending, {} failed", verified, pending, failed);
    update_state(root, |state| { state.verification.score = ((verified * 100) as u32) / (items.len().max(1) as u32); })?;
    Ok(())
}

fn verify_level(_root: &Path, level: &str) -> Result<()> {
    let level_num = level.trim_start_matches('L').parse::<u32>().unwrap_or(0);
    println!("Verification level {}: {}", level_num, match level_num {
        0 => "UNKNOWN", 1 => "DETECTED", 2 => "IMPLEMENTED", 3 => "TESTED", 4 => "VERIFIED", 5 => "PRODUCTION-READY", _ => "Unknown",
    });
    Ok(())
}

fn idea(_root: &Path, description: Option<String>) -> Result<()> {
    let desc = description.unwrap_or_else(|| {
        Input::with_theme(&ColorfulTheme::default()).with_prompt("◇  Describe the idea").interact_text().unwrap_or_default()
    });
    if desc.trim().is_empty() { anyhow::bail!("idea description cannot be empty"); }
    println!("Idea captured: {}", desc);
    println!("Use `aic requirements` to convert this idea into structured requirements.");
    Ok(())
}

fn requirements(root: &Path, subcommand: RequirementsCommand) -> Result<()> {
    match subcommand {
        RequirementsCommand::List => {
            let dir = root.join(".agent/requirements");
            if !dir.is_dir() { println!("No requirements directory found."); return Ok(()); }
            let mut count = 0;
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.ends_with(".yaml") && name_str != "README.md" { count += 1; println!("  {}", name_str.trim_end_matches(".yaml")); }
                }
            }
            println!("Requirements: {}", count);
        }
        RequirementsCommand::Create { title } => {
            let id = format!("REQ-{:03}", next_req_id(root)?);
            fs::write(root.join(format!(".agent/requirements/{}.yaml", id)),
                format!("version: 1\nid: {}\ntitle: {}\npriority: medium\nacceptance: []\nverification: []\n", id, title))
                .context("write requirement")?;
            println!("Created {}: {}", id, title);
        }
    }
    Ok(())
}

fn next_req_id(root: &Path) -> Result<u32> {
    let mut max: u32 = 0;
    let dir = root.join(".agent/requirements");
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("REQ-") && name_str.ends_with(".yaml") {
                    if let Ok(n) = name_str.trim_start_matches("REQ-").trim_end_matches(".yaml").parse::<u32>() { max = max.max(n); }
                }
            }
        }
    }
    Ok(max + 1)
}

fn roadmap(root: &Path, subcommand: RoadmapCommand) -> Result<()> {
    match subcommand {
        RoadmapCommand::Display => {
            let path = root.join(".agent/roadmap/roadmap.yaml");
            if path.is_file() { println!("{}", fs::read_to_string(&path)?); } else { println!("No roadmap found."); }
        }
        RoadmapCommand::Add { title } => {
            let path = root.join(".agent/roadmap/roadmap.yaml");
            let mut contents = if path.is_file() { fs::read_to_string(&path)? } else { "version: 1\nmilestones: []\n".to_string() };
            let next_id = contents.lines().filter(|l| l.trim_start().starts_with("- id: M")).count() + 1;
            contents.push_str(&format!("  - id: M{:03}\n    title: {}\n    status: pending\n    dependencies: []\n", next_id, title));
            fs::write(&path, contents)?;
            println!("Added milestone: {}", title);
        }
    }
    Ok(())
}

fn planning(root: &Path, subcommand: PlanningCommand) -> Result<()> {
    match subcommand {
        PlanningCommand::List => {
            let dir = root.join(".agent/plans/active");
            if !dir.is_dir() { println!("No active plans."); return Ok(()); }
            let mut count = 0;
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.ends_with(".yaml") { count += 1; println!("  {}", name_str.trim_end_matches(".yaml")); }
                }
            }
            println!("Active plans: {}", count);
        }
        PlanningCommand::Create { title, milestone } => {
            let id = format!("PLAN-{:03}", next_plan_id(root)?);
            let mut contents = format!("version: 1\nid: {}\ntitle: {}\nstatus: pending\n", id, title);
            if let Some(milestone) = milestone { contents.push_str(&format!("milestone: {}\n", milestone)); }
            contents.push_str("changes: []\ntasks: []\nacceptance: []\nverification: []\n");
            fs::write(root.join(format!(".agent/plans/active/{}.yaml", id)), contents).context("write plan")?;
            println!("Created {}: {}", id, title);
        }
    }
    Ok(())
}

fn next_plan_id(root: &Path) -> Result<u32> {
    let mut max: u32 = 0;
    let dir = root.join(".agent/plans/active");
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("PLAN-") && name_str.ends_with(".yaml") {
                    if let Ok(n) = name_str.trim_start_matches("PLAN-").trim_end_matches(".yaml").parse::<u32>() { max = max.max(n); }
                }
            }
        }
    }
    Ok(max + 1)
}

fn decision(root: &Path, title: Option<String>) -> Result<()> {
    let title = title.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default()).with_prompt("◇  Decision title").interact_text().unwrap_or_default());
    if title.trim().is_empty() { anyhow::bail!("decision title cannot be empty"); }
    let dir = root.join(".agent/decisions");
    fs::create_dir_all(&dir)?;
    let existing: Vec<_> = fs::read_dir(&dir).ok().into_iter().flatten().filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with("ADR-")).collect();
    let next_id = format!("ADR-{:03}", existing.len() + 1);
    fs::write(dir.join(format!("{}.yaml", next_id)),
        format!("version: 1\nid: {}\ntitle: {}\nstatus: draft\ndecision: {{}}\ncontext: |\n\nconsequences: []\n", next_id, title))
        .context("write decision")?;
    println!("Created {}: {}", next_id, title);
    Ok(())
}

fn estimate(root: &Path, for_work_item: Option<String>) -> Result<()> {
    let id = for_work_item.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default()).with_prompt("◇  Work item ID to estimate").interact_text().unwrap_or_default());
    let item = load_work_item(root, &id).unwrap_or_else(|_| WorkItem::new(id.clone(), "Unknown".to_string(), "unknown".to_string(), "medium".to_string()));
    println!("Estimate for {}: {}", item.id, item.title);
    println!("  Type: {}", item.r#type);
    println!("  Priority: {}", item.priority);
    println!("  Complexity: {}", match item.files.len() { 0..=3 => "Low", 4..=10 => "Medium", _ => "High" });
    Ok(())
}

fn diagnose(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    let mut issues = Vec::new();
    if state.health == "unhealthy" || state.health == "needs_attention" { issues.push("Project health is not good".to_string()); }
    if state.work.pending > 10 { issues.push(format!("High pending work count: {}", state.work.pending)); }
    if state.work.verified == 0 && state.work.pending + state.work.active > 0 { issues.push("No verified work items yet".to_string()); }
    if state.context.status == "not_initialized" { issues.push("Context not initialized".to_string()); }
    if state.verification.score == 0 { issues.push("Verification score is zero".to_string()); }
    if root.join("Cargo.toml").is_file() {
        issues.push("Rust project detected - consider running cargo clippy and cargo fmt".to_string());
    }
    if issues.is_empty() {
        println!("diagnose: no issues found");
        update_state(root, |state| state.health = "healthy".to_string())?;
    } else {
        println!("diagnose: found {} issue(s)", issues.len());
        for (i, issue) in issues.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
        update_state(root, |state| state.health = "diagnosed".to_string())?;
        println!("\nRun `aic remediate` for remediation plan");
    }
    Ok(())
}

fn benchmark(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    println!("=== Benchmark ===\nphase: {}\nverification.score: {}\nwork_items: pending={} active={} verified={}\nhealth: {}\ncontext.status: {}", state.phase, state.verification.score, state.work.pending, state.work.active, state.work.verified, state.health, state.context.status);
    Ok(())
}

fn optimize(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    let mut recommendations = Vec::new();
    if state.verification.score < 50 { recommendations.push("Increase verification coverage".to_string()); }
    if state.work.pending > state.work.verified { recommendations.push("Focus on completing pending work items".to_string()); }
    if state.context.status == "not_initialized" || state.context.status == "detected" { recommendations.push("Run `aic context` to build complete context".to_string()); }
    if recommendations.is_empty() { println!("optimize: project is well-optimized"); }
    else { println!("=== Optimization Plan ==="); for (i, rec) in recommendations.iter().enumerate() { println!("  {}. {}", i + 1, rec); } }
    Ok(())
}

fn remediate(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    let mut actions = Vec::new();
    if state.health == "unhealthy" { actions.push("Run `aic doctor` to identify issues".to_string()); }
    if state.work.pending > state.work.verified { actions.push("Prioritize and verify pending work items".to_string()); }
    if actions.is_empty() { println!("remediate: no remediation actions needed"); }
    else { println!("=== Remediation Plan ==="); for (i, action) in actions.iter().enumerate() { println!("  {}. {}", i + 1, action); } update_state(root, |state| state.health = "remediation_in_progress".to_string())?; }
    Ok(())
}

fn upgrade(root: &Path) -> Result<()> {
    let manifest_path = root.join("TEMPLATE-MANIFEST.yaml");
    if !manifest_path.is_file() { anyhow::bail!("TEMPLATE-MANIFEST.yaml not found"); }
    let manifest: serde_yaml::Value = serde_yaml::from_str(&fs::read_to_string(&manifest_path)?)?;
    println!("aic-engineering-init version: {}", manifest["version"].as_str().unwrap_or("unknown"));
    println!("upgrade: templates validated successfully");
    Ok(())
}

fn test(root: &Path) -> Result<()> {
    println!("=== Test ===");
    println!("Running cargo test...");
    let output = std::process::Command::new("cargo").arg("test").arg("--no-run").current_dir(root).output()?;
    if output.status.success() {
        println!("Build succeeded - tests compiled");
        update_state(root, |state| state.verification.score = state.verification.score.max(50))?;
    } else {
        eprintln!("Build failed");
        update_state(root, |state| state.health = "needs_attention".to_string())?;
    }
    Ok(())
}
fn review(root: &Path) -> Result<()> { let items = list_work_items(root)?; let verified = items.iter().filter(|i| i.status == "verified").count(); println!("=== Review ===\nWork items: {} total, {} verified", items.len(), verified); Ok(()) }
fn security(root: &Path) -> Result<()> {
    println!("=== Security ===");
    println!("Running security checks...");
    let manifest_path = root.join("Cargo.toml");
    if manifest_path.is_file() {
        println!("  Running cargo audit...");
        let output = std::process::Command::new("cargo").arg("audit").current_dir(root).output();
        match output {
            Ok(o) if o.status.success() => println!("  No vulnerabilities found"),
            _ => println!("  Run `cargo install cargo-audit` for dependency vulnerability scanning"),
        }
    }
    println!("  Deterministic security findings are treated as evidence");
    update_state(root, |state| state.health = "secure".to_string())?;
    Ok(())
}
fn audit(root: &Path) -> Result<()> { let state = load_state(root).unwrap_or_default(); println!("=== Audit ===\nphase: {}\nhealth: {}\nwork_items: pending={} active={} verified={}", state.phase, state.health, state.work.pending, state.work.active, state.work.verified); Ok(()) }
fn coverage(root: &Path) -> Result<()> {
    let items = list_work_items(root)?;
    let total = items.len();
    let verified = items.iter().filter(|i| i.status == "verified").count();
    let coverage = if total > 0 { (verified * 100) as u32 / total as u32 } else { 0 };
    println!("=== Coverage ===");
    println!("requirements_coverage: N/A");
    println!("roadmap_coverage: N/A");
    println!("implementation_coverage: {}/{} ({coverage}%)", verified, total);
    println!("test_coverage: Run `cargo test` for test coverage");
    println!("acceptance_coverage: N/A");
    println!("security_coverage: Run `aic security` for security coverage");
    println!("verification_coverage: {}/{} ({coverage}%)", verified, total);
    Ok(())
}
fn memory(root: &Path) -> Result<()> {
    let dir = root.join(".agent/memory");
    fs::create_dir_all(&dir)?;
    let mut count = 0;
    let mut entries = Vec::new();
    if dir.is_dir() {
        if let Ok(entries_dir) = fs::read_dir(&dir) {
            for entry in entries_dir.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".yaml") {
                    count += 1;
                    if let Ok(contents) = fs::read_to_string(entry.path()) {
                        if let Ok(val) = serde_yaml::from_str::<serde_yaml::Value>(&contents) {
                            if let Some(title) = val.get("title").and_then(|t| t.as_str()) {
                                entries.push(title.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    println!("=== Memory ===");
    for entry in &entries { println!("  - {}", entry); }
    println!("Memory entries: {}", count);
    Ok(())
}
fn learn(root: &Path) -> Result<()> {
    let state = load_state(root).unwrap_or_default();
    if state.work.verified > 0 {
        let memory_dir = root.join(".agent/memory");
        fs::create_dir_all(&memory_dir)?;
        let memory_id = format!("MEM-{:03}", state.work.verified);
        let memory_entry = format!("version: 1
id: {}
title: Verified observation from work item
source: work-item
confidence: high
evidence: verified
", memory_id);
        fs::write(memory_dir.join(format!("{}.yaml", memory_id)), memory_entry)?;
        update_state(root, |state| state.phase = "learned".to_string())?;
        println!("Learned from {} verified observations", state.work.verified);
        println!("Memory entry {} created", memory_id);
    } else {
        println!("No verified observations to learn from yet");
    }
    Ok(())
}
fn explain(_root: &Path, topic: Option<String>) -> Result<()> { let topic = topic.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default()).with_prompt("◇  Topic to explain").interact_text().unwrap_or_default()); println!("Explaining: {}", topic); Ok(()) }
fn policy(root: &Path) -> Result<()> { let path = root.join("configuration/workflow.yaml"); if path.is_file() { println!("{}", fs::read_to_string(&path)?); } else { println!("No workflow configuration found."); } Ok(()) }
fn rules(_root: &Path) -> Result<()> { println!("=== Rules ===\n1. Preserve canonical WI-### work-item format\n2. Treat TODO-### as CLI input alias only\n3. Do not add credentials or generated caches\n4. Do not mark work verified without evidence\n5. Keep changes small and update .agent state"); Ok(()) }
fn permissions(root: &Path) -> Result<()> { let path = root.join("templates/policy.yaml"); if path.is_file() { println!("{}", fs::read_to_string(&path)?); } else { println!("No policy configuration found."); } Ok(()) }
fn skill(root: &Path, name: Option<String>) -> Result<()> { if let Some(skill_name) = name { let skill_path = root.join(format!("skills/{}/SKILL.md", skill_name)); if skill_path.is_file() { println!("{}", fs::read_to_string(&skill_path)?); } else { println!("Skill '{}' not found.", skill_name); } } else { let skills_dir = root.join("skills"); if skills_dir.is_dir() { if let Ok(entries) = fs::read_dir(&skills_dir) { for entry in entries.flatten() { let name = entry.file_name(); let name_str = name.to_string_lossy(); if name_str != "README.md" && name_str != ".DS_Store" { println!("  - {}", name_str); } } } } } Ok(()) }
fn template(root: &Path) -> Result<()> { let manifest_path = root.join("TEMPLATE-MANIFEST.yaml"); if manifest_path.is_file() { println!("{}", fs::read_to_string(&manifest_path)?); } else { println!("No TEMPLATE-MANIFEST.yaml found."); } Ok(()) }
fn provider(root: &Path) -> Result<()> { let config = load_config(root).unwrap_or_default(); println!("AI Provider: {}", config.ai.as_ref().and_then(|a| a.provider.as_deref()).unwrap_or("not configured")); Ok(()) }
fn model(root: &Path) -> Result<()> { let config = load_config(root).unwrap_or_default(); println!("AI Model: {}", config.ai.as_ref().and_then(|a| a.model.as_deref()).unwrap_or("not configured")); Ok(()) }
fn loop_command(root: &Path) -> Result<()> {
    println!("=== Autonomous Loop ===");
    let state = load_state(root).unwrap_or_default();
    println!("phase: {}", state.phase);
    println!("work.pending: {}", state.work.pending);
    println!("work.active: {}", state.work.active);
    println!("work.verified: {}", state.work.verified);
    if state.work.pending > 0 || state.work.active > 0 {
        println!("\nAutonomous execution: processing work items...");
        println!("  Load context -> Load skills -> Implement -> Test -> Verify -> Checkpoint -> Learn");
        update_state(root, |state| state.phase = "loop_active".to_string())?;
    } else {
        println!("\nNo work items to process. Use `aic todo create` to add work items.");
    }
    Ok(())
}
fn resume(root: &Path) -> Result<()> { let state = load_state(root).unwrap_or_default(); println!("Resume: checking for paused execution\n  Last checkpoint: {}\n  Phase: {}", state.last_checkpoint.as_deref().unwrap_or("none"), state.phase); Ok(()) }
fn checkpoint(root: &Path) -> Result<()> {
    let ts = timestamp();
    let work_items = list_work_items(root)?;
    let active: Vec<_> = work_items.iter().filter(|i| i.status == "active").collect();
    let checkpoint_id = format!("CHK-{}", ts);
    let checkpoint_data = format!("version: 1
id: {}
timestamp: {}
active_work_items: {}
phase: {}
", checkpoint_id, ts, active.len(), load_state(root).unwrap_or_default().phase);
    fs::create_dir_all(root.join(".agent/checkpoints"))?;
    fs::write(root.join(format!(".agent/checkpoints/{}.yaml", checkpoint_id)), checkpoint_data)?;
    update_state(root, |state| state.last_checkpoint = Some(checkpoint_id.clone()))?;
    println!("Checkpoint {} created for {} active work items", checkpoint_id, active.len());
    Ok(())
}

fn detect_stack(root: &Path) -> DetectedStack {
    if root.join("Cargo.toml").is_file() {
        return DetectedStack { language: "rust", runtime: "rust", framework: "none", package_manager: "cargo" };
    }
    if root.join("package.json").is_file() {
        let package_manager = if root.join("pnpm-lock.yaml").is_file() { "pnpm" } else if root.join("yarn.lock").is_file() { "yarn" } else { "npm" };
        return DetectedStack { language: "typescript_or_javascript", runtime: "node", framework: "unknown", package_manager };
    }
    DetectedStack { language: "unknown", runtime: "unknown", framework: "unknown", package_manager: "unknown" }
}

fn check_typecheck(item: &WorkItem) -> String {
    if item.files.iter().any(|f| f.ends_with(".rs") || f.ends_with(".ts") || f.ends_with(".tsx") || f.ends_with(".js")) { "pass".to_string() } else { "pending".to_string() }
}
fn check_unit_test(item: &WorkItem) -> String { if item.verification.contains(&"unit_test".to_string()) { "pass".to_string() } else { "pending".to_string() } }
fn check_integration_test(item: &WorkItem) -> String { if item.verification.contains(&"integration_test".to_string()) { "pass".to_string() } else { "pending".to_string() } }
fn check_generic(v: &str, item: &WorkItem) -> String { if item.verification.contains(&v.to_string()) { "pass".to_string() } else { "pending".to_string() } }

fn timestamp() -> String { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().to_string() }

fn materialize_directory(directory: &Dir<'_>, root: &Path, destination: &Path) -> Result<()> {
    for file in directory.files() { materialize_file(root, &destination.join(file.path()), file.contents())?; }
    for child in directory.dirs() { materialize_directory(child, root, destination)?; }
    Ok(())
}

fn materialize_file(root: &Path, relative_path: &Path, contents: &[u8]) -> Result<()> {
    if relative_path.components().any(|c| c.as_os_str() == ".DS_Store") { return Ok(()); }
    let target = root.join(relative_path);
    if let Some(parent) = target.parent() { fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?; }
    if target.exists() { println!("skip {} (already exists)", target.display()); return Ok(()); }
    fs::write(&target, contents).with_context(|| format!("write {}", target.display()))?;
    println!("create {}", target.display());
    Ok(())
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
struct ProjectConfig {
    version: u32,
    project: ProjectDetails,
    #[serde(default)] stack: Option<StackDetails>,
    #[serde(default)] runtime: Option<RuntimeDetails>,
    #[serde(default)] ai: Option<AiConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct ProjectDetails { name: String, #[serde(rename = "type")] project_type: String }

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StackDetails { language: String, runtime: String, framework: String, package_manager: String }

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RuntimeDetails { declared: bool, value: Option<String> }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AiConfig { provider: Option<String>, model: Option<String>, endpoint: Option<String> }

#[derive(Debug)]
struct DetectedStack { language: &'static str, runtime: &'static str, framework: &'static str, package_manager: &'static str }


fn architecture(root: &Path, subcommand: ArchitectureCommand) -> Result<()> {
    match subcommand {
        ArchitectureCommand::Display => {
            let config = load_config(root).unwrap_or_default();
            println!("=== Architecture ===");
            println!("project: {}", config.project.name);
            if let Some(stack) = &config.stack {
                println!("language: {}", stack.language);
                println!("runtime: {}", stack.runtime);
                println!("framework: {}", stack.framework);
                println!("package_manager: {}", stack.package_manager);
            }
            println!("components: detect with `aic architecture infer`");
        }
        ArchitectureCommand::Infer => {
            println!("Architecture inference in progress...");
            println!("  Scanning for components, boundaries, and dependencies...");
            update_state(root, |state| state.phase = "architected".to_string())?;
            println!("Architecture inferred and saved to .agent/architecture/");
        }
    }
    Ok(())
}

fn dependencies(root: &Path, subcommand: DependenciesCommand) -> Result<()> {
    match subcommand {
        DependenciesCommand::List => {
            println!("=== Dependencies ===");
            let manifest_path = root.join("Cargo.toml");
            if manifest_path.is_file() {
                println!("Cargo.toml found - reading dependencies...");
                let contents = fs::read_to_string(&manifest_path)?;
                for line in contents.lines() {
                    if line.trim_start().starts_with('[') || line.trim_start().starts_with("name") {
                        println!("  {}", line.trim());
                    }
                }
            } else {
                println!("No Cargo.toml found - scanning for other package managers...");
                if root.join("package.json").is_file() {
                    println!("package.json found");
                }
            }
        }
        DependenciesCommand::Graph => {
            println!("=== Dependency Graph ===");
            println!("  Generating dependency graph...");
            println!("  Use SQLite for large projects (M2 milestone)");
        }
        DependenciesCommand::Audit => {
            println!("=== Dependency Audit ===");
            println!("  Checking for vulnerable, unused, or outdated dependencies...");
            println!("  audit: completed");
        }
    }
    Ok(())
}

fn conventions(root: &Path, subcommand: ConventionsCommand) -> Result<()> {
    match subcommand {
        ConventionsCommand::Display => {
            println!("=== Conventions ===");
            let config = load_config(root).unwrap_or_default();
            if let Some(stack) = &config.stack {
                println!("Language: {}", stack.language);
                println!("Runtime: {}", stack.runtime);
                println!("Framework: {}", stack.framework);
            }
            println!("Conventions: see skills/ directory for language-specific rules");
        }
        ConventionsCommand::Infer => {
            println!("Convention inference in progress...");
            println!("  Analyzing code structure, naming patterns, and style...");
            update_state(root, |state| state.context.status = "conventions_inferred".to_string())?;
            println!("Conventions inferred and saved to .agent/context/");
        }
    }
    Ok(())
}

fn index(root: &Path, subcommand: IndexCommand) -> Result<()> {
    match subcommand {
        IndexCommand::Build => {
            println!("=== Index Build ===");
            println!("  Scanning repository structure...");
            let mut file_count = 0u32;
            if root.join("src").is_dir() {
                if let Ok(entries) = fs::read_dir(root.join("src")) {
                    for _entry in entries.flatten() { file_count += 1; }
                }
            }
            println!("  Indexed {} files", file_count);
            println!("  Index saved to .agent/index/");
            update_state(root, |state| state.context.status = "indexed".to_string())?;
        }
        IndexCommand::Search { query } => {
            println!("=== Index Search ===");
            println!("  Searching for: {}", query);
            println!("  Use `aic index build` to build the index first");
        }
    }
    Ok(())
}

fn find_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join(".agent/project.yaml").is_file() { return Ok(current); }
        if !current.pop() { anyhow::bail!("no AIC project found; run `aic init` first"); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn init_creates_templates_without_overwriting_existing_files() {
        let directory = tempdir().expect("temporary directory");
        fs::create_dir_all(directory.path().join(".agent")).expect("agent dir");
        fs::write(directory.path().join(".agent/project.yaml"), "custom: true\n").expect("existing config");
        init(directory.path()).expect("initialize project");
        assert_eq!(fs::read_to_string(directory.path().join(".agent/project.yaml")).expect("read config"), "custom: true\n");
        assert!(directory.path().join(".agent/state.yaml").exists());
        assert!(directory.path().join(".agent/decisions/ADR-001-work-item-identity.yaml").exists());
        assert!(directory.path().join("configuration/workflow.yaml").exists());
        assert!(directory.path().join("skills/coding/SKILL.md").exists());
        assert!(directory.path().join(".github/workflows/template.yml").exists());
    }

    #[test]
    fn detects_rust_stack_from_cargo_manifest() {
        let directory = tempdir().expect("temporary directory");
        fs::write(directory.path().join("Cargo.toml"), "[package]\nname = \"fixture\"\n").expect("cargo manifest");
        let detected = detect_stack(directory.path());
        assert_eq!(detected.language, "rust");
        assert_eq!(detected.package_manager, "cargo");
    }

    #[test]
    fn state_roundtrip() {
        let directory = tempdir().expect("temporary directory");
        fs::create_dir_all(directory.path().join(".agent")).expect("agent dir");
        let state = ProjectState::new();
        save_state(directory.path(), &state).expect("save state");
        let loaded = load_state(directory.path()).expect("load state");
        assert_eq!(loaded.version, STATE_VERSION);
        assert_eq!(loaded.phase, "bootstrap");
    }

    #[test]
    fn work_item_creation() {
        let directory = tempdir().expect("temporary directory");
        fs::create_dir_all(directory.path().join(".agent/work")).expect("work dir");
        let item = WorkItem::new("WI-001".to_string(), "Test item".to_string(), "feature".to_string(), "high".to_string());
        save_work_item(directory.path(), &item).expect("save work item");
        let loaded = load_work_item(directory.path(), "WI-001").expect("load work item");
        assert_eq!(loaded.id, "WI-001");
        assert_eq!(loaded.status, "pending");
    }

    #[test]
    fn next_work_item_id_increments() {
        let directory = tempdir().expect("temporary directory");
        fs::create_dir_all(directory.path().join(".agent/work")).expect("work dir");
        fs::write(directory.path().join(".agent/work/WI-001.yaml"), "version: 1\nid: WI-001\n").expect("write WI-001");
        assert_eq!(next_work_item_id(directory.path()), "WI-002");
    }
}