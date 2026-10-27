mod cargo;
mod cli;
mod diagnostic;
mod output;
mod render;
mod source;
use diagnostic::model::*;
fn run(opts: cli::RunOptions) -> Result<i32, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let ctx = cli::cargo_context(
        &opts,
        &cwd,
        std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
    )?;
    let project = cargo::load_project(&ctx)?;
    let output = opts
        .output
        .as_ref()
        .map_or_else(|| project.target_dir.join("clippy-html"), |p| cwd.join(p));
    let target = output::prepare_target(&output, &project)?;
    let collected = cargo::collect(&ctx, &opts)?;
    let raw_count = collected.messages.len();
    let findings = diagnostic::dedup::merge_findings(
        collected
            .messages
            .iter()
            .map(|m| diagnostic::normalize::normalize(m, &project.workspace_root))
            .collect(),
    );
    let report = Report {
        project,
        execution: collected.execution,
        raw_count,
        findings,
    };
    let sources = source::load_sources(&report.project, &report.findings);
    let pages = render::paths::build_page_map(&report, &sources);
    let published = output::publish_report(&target, |dir| {
        render::render_report(&report, &sources, &pages, dir)
    })
    .map_err(|e| {
        format!(
            "{e}\nClippy exit: {:?}; collection complete: {}",
            report.execution.code, report.execution.complete
        )
    })?;
    for warning in published.warnings {
        eprintln!("Warning: {warning}");
    }
    println!("Report: {}", published.index.display());
    if opts.open {
        let url = url::Url::from_file_path(&published.index)
            .map_err(|_| "Cannot form report file URL")?;
        if let Err(e) = webbrowser::open(url.as_str()) {
            eprintln!("Warning: report generated, browser could not open it: {e}");
        }
    }
    Ok(report.execution.exit_code())
}
fn main() {
    let opts = match cli::parse_args(std::env::args_os().collect()) {
        Ok(o) => o,
        Err(e) => e.exit(),
    };
    let code = match run(opts) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("cargo-clippy-html: {e}");
            1
        }
    };
    std::process::exit(code);
}
