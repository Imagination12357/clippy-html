//! Report data shared by normalization, source loading and rendering.
use serde::{Deserialize, Serialize};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TargetInfo {
    pub name: String,
    pub kinds: Vec<String>,
    pub source: PathBuf,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnippetLine {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Span {
    pub path: PathBuf,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub primary: bool,
    pub label: Option<String>,
    pub text: Vec<SnippetLine>,
    pub expansion: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note {
    pub level: String,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Note>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Replacement {
    pub span: Span,
    pub text: String,
    pub applicability: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Suggestion {
    pub message: String,
    pub child_path: Vec<usize>,
    pub replacements: Vec<Replacement>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Finding {
    pub id: String,
    pub package_id: String,
    pub targets: Vec<TargetInfo>,
    pub origin: String,
    pub code: Option<String>,
    pub level: String,
    pub category: Option<String>,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Note>,
    pub suggestions: Vec<Suggestion>,
    pub rendered: Option<String>,
}
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest: PathBuf,
    pub workspace_member: bool,
}
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub workspace_root: PathBuf,
    pub target_dir: PathBuf,
    pub packages: Vec<PackageInfo>,
}
#[derive(Debug, Clone)]
pub struct Execution {
    pub code: Option<i32>,
    pub complete: bool,
    pub requested_args: Vec<OsString>,
    pub observed_packages: Vec<String>,
    pub observed_targets: Vec<TargetInfo>,
    pub stderr: String,
    pub stderr_truncated: bool,
    pub text_stdout: String,
    pub stdout_truncated: bool,
    pub issues: Vec<String>,
}
impl Execution {
    pub fn exit_code(&self) -> i32 {
        match self.code {
            Some(0) if self.complete => 0,
            Some(n) if n != 0 => n,
            _ => 1,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CategoryMetadata {
    pub mode: String,
    pub selector: String,
    pub url: String,
    pub source_hash: String,
    pub table_hash: String,
    pub reason: Option<String>,
}
pub struct Report {
    pub category_metadata: Option<CategoryMetadata>,
    pub project: ProjectInfo,
    pub execution: Execution,
    pub raw_count: usize,
    pub findings: Vec<Finding>,
}
