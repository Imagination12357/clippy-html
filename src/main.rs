mod cargo;
mod cli;
mod diagnostic;
mod lint;
mod metadata;
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
    let metadata_target = opts
        .metadata
        .as_ref()
        .map(|path| metadata::MetadataTarget::prepare(&cwd.join(path), &target))
        .transpose()?;
    let collected = cargo::collect(&ctx, &opts)?;
    let raw_count = collected.messages.len();
    let mut findings = diagnostic::dedup::merge_findings(
        collected
            .messages
            .iter()
            .map(|m| diagnostic::normalize::normalize(m, &project.workspace_root))
            .collect(),
    );
    let category_metadata = lint::metadata::resolve(&mut findings, opts.offline);
    if let Some(metadata) = &category_metadata
        && let Some(reason) = &metadata.reason
        && !opts.offline
    {
        eprintln!(
            "Warning: category metadata uses bundled {} data: {reason}",
            metadata.selector
        );
    }
    let report = Report {
        category_metadata,
        project,
        execution: collected.execution,
        raw_count,
        findings,
    };
    let sources = source::load_sources(&report.project, &report.findings);
    let pages = render::paths::build_page_map(&report, &sources);
    let navigation = metadata_target
        .as_ref()
        .map(|_| metadata::build(&report, &sources, &pages));
    let prepared_metadata = metadata_target
        .zip(navigation.as_ref())
        .map(|(target, navigation)| target.stage(navigation.json()?))
        .transpose()?;
    let published = output::publish_report(&target, |dir| {
        render::render_report(&report, &sources, &pages, dir)?;
        if let Some(metadata) = &prepared_metadata {
            metadata.write_report(dir)?;
        }
        if let Some(navigation) = &navigation {
            navigation.validate(dir)?;
        }
        Ok(())
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
    if let Some(metadata) = prepared_metadata {
        metadata.publish_external().map_err(|e| format!(
            "{e}\nHTML report published at {}; requested metadata was not published.\nClippy exit: {:?}; collection complete: {}",
            published.index.display(), report.execution.code, report.execution.complete
        ))?;
        println!(
            "Metadata: {}",
            cwd.join(opts.metadata.as_ref().unwrap()).display()
        );
    }
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
