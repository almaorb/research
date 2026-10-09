//! What a session is for, chosen when it starts.
//!
//! - **Research**: answer a question with a sourced report. It runs straight
//!   through in auto mode: no plan to approve, no build supervisor. It writes its
//!   deliverables into the project's artifacts folder (passed to the agent as a
//!   writable directory). It loads the `alma-report` skill and follows the
//!   playbook's research contract: sources with publisher and dates, a sources
//!   list, a check of its own claims.
//! - **Plan**: research an engineering goal and end in a plan the Alma IDE's
//!   supervisor builds (`alma-research` + `alma-plan`, plan mode, Approve and
//!   build).
//! - No kind (older sessions, and the dashboard's plain composer): everything
//!   as before.
//!
//! A session may also carry **reference folders**: other repositories (or any
//! folder) it reads beside its own worktree, e.g. the app's code when the
//! project is its business plan. Claude Code gets each one as an `--add-dir`
//! with edits denied; every harness sees them listed in the playbook.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

/// The most reference folders one session may carry.
pub const MAX_REFERENCE_DIRS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskKind {
    Research,
    Plan,
}

impl TaskKind {
    pub fn from_id(id: &str) -> Option<TaskKind> {
        match id.trim() {
            "research" | "report" => Some(TaskKind::Research),
            "plan" => Some(TaskKind::Plan),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            TaskKind::Research => "research",
            TaskKind::Plan => "plan",
        }
    }

    /// The Claude permission mode a session of this kind starts in when none
    /// was chosen: research runs unattended, a plan waits for approval.
    pub fn default_claude_permission(self) -> &'static str {
        match self {
            TaskKind::Research => "auto",
            TaskKind::Plan => "plan",
        }
    }
}

/// The kind stored on a session, if any (an unknown stored value is none).
pub fn of(stored: Option<&str>) -> Option<TaskKind> {
    stored.and_then(TaskKind::from_id)
}

/// Reference folders as sent: absolute, existing directories, each once, at
/// most [`MAX_REFERENCE_DIRS`], canonicalized. `own_repo` (the project's own
/// repository) is dropped silently: the worktree already is it.
pub fn check_reference_dirs(dirs: &[String], own_repo: &Path) -> Result<Vec<String>> {
    let own = std::fs::canonicalize(own_repo).unwrap_or_else(|_| own_repo.to_path_buf());
    let mut out: Vec<String> = Vec::new();
    for raw in dirs {
        let raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        let path = PathBuf::from(raw);
        if !path.is_absolute() {
            return Err(anyhow!("reference folders are absolute paths: {raw}"));
        }
        let canon = std::fs::canonicalize(&path).map_err(|_| anyhow!("no such folder: {raw}"))?;
        if !canon.is_dir() {
            return Err(anyhow!("not a folder: {raw}"));
        }
        if canon == own {
            continue;
        }
        let canon = canon.to_string_lossy().into_owned();
        if !out.contains(&canon) {
            out.push(canon);
        }
    }
    if out.len() > MAX_REFERENCE_DIRS {
        return Err(anyhow!("at most {MAX_REFERENCE_DIRS} reference folders"));
    }
    Ok(out)
}

/// Stored reference folders (a JSON list) back to a list; anything malformed is none.
pub fn parse_reference_dirs(json: Option<&str>) -> Vec<String> {
    json.and_then(|j| serde_json::from_str::<Vec<String>>(j).ok())
        .unwrap_or_default()
}

/// The playbook section for a session: what it is for and what it reads.
pub fn playbook_section(
    kind: Option<TaskKind>,
    reference_dirs: &[String],
    artifacts: &str,
) -> String {
    let mut out = String::new();
    match kind {
        Some(TaskKind::Research) => out.push_str(&format!(
            "## This task: research → a sourced report\n\n\
             This session is a **research task**, not an engineering build. Answer the \
             question with evidence and write the deliverables; there is no plan to \
             present and nothing to approve. Do not call ExitPlanMode and do not use the \
             `alma-plan` skill. Load **`alma-report`** before you start: it is the method \
             (questions, search rounds, source rules, verification) and the deliverable \
             layout.\n\n\
             - Write every deliverable under `{artifacts}` (a folder per report), never \
             into the repository unless the user asks.\n\
             - Every factual claim from the web carries its source: the link, the \
             publisher, the date it was published and the date you read it. Keep the \
             list in `sources.jsonl` beside the report.\n\
             - Change no code in this or any reference repository.\n\n"
        )),
        Some(TaskKind::Plan) => out.push_str(
            "## This task: an engineering plan\n\n\
             This session researches an engineering goal and ends in a plan the Alma \
             IDE's supervisor builds. Load **`alma-research`**, then **`alma-plan`**, and \
             present the plan with ExitPlanMode for the user to approve.\n\n",
        ),
        None => {}
    }
    if !reference_dirs.is_empty() {
        out.push_str(
            "## Reference folders\n\n\
             Besides your worktree, read these folders (absolute paths). They are for \
             reading: never edit, commit or run anything that changes them.\n\n",
        );
        for dir in reference_dirs {
            out.push_str(&format!("- `{dir}`\n"));
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_round_trip_and_unknown_is_none() {
        assert_eq!(TaskKind::from_id("research"), Some(TaskKind::Research));
        assert_eq!(TaskKind::from_id("report"), Some(TaskKind::Research));
        assert_eq!(TaskKind::from_id(" plan "), Some(TaskKind::Plan));
        assert_eq!(TaskKind::from_id("build"), None);
        assert_eq!(of(Some("research")).map(TaskKind::id), Some("research"));
        assert_eq!(of(Some("nope")), None);
        assert_eq!(TaskKind::Research.default_claude_permission(), "auto");
        assert_eq!(TaskKind::Plan.default_claude_permission(), "plan");
    }

    /// A fresh empty folder under the system temp dir.
    fn temp_folder(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("orx-task-kind-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn reference_dirs_are_absolute_existing_and_distinct() {
        let a = temp_folder("a");
        let b = temp_folder("b");
        let own = temp_folder("own");
        let pa = a.to_string_lossy().into_owned();
        let pb = b.to_string_lossy().into_owned();
        let po = own.to_string_lossy().into_owned();
        let got = check_reference_dirs(
            &[pa.clone(), pb.clone(), pa.clone(), po, "  ".into()],
            own.as_path(),
        )
        .unwrap();
        assert_eq!(
            got.len(),
            2,
            "duplicates, blanks and the own repo are dropped: {got:?}"
        );
        assert!(check_reference_dirs(&["relative/path".into()], own.as_path()).is_err());
        assert!(check_reference_dirs(&["/no/such/folder/anywhere".into()], own.as_path()).is_err());
        let file = a.as_path().join("f.txt");
        std::fs::write(&file, "x").unwrap();
        assert!(
            check_reference_dirs(&[file.to_string_lossy().into_owned()], own.as_path()).is_err()
        );
        let many: Vec<String> = (0..=MAX_REFERENCE_DIRS)
            .map(|i| {
                let d = a.as_path().join(format!("d{i}"));
                std::fs::create_dir_all(&d).unwrap();
                d.to_string_lossy().into_owned()
            })
            .collect();
        assert!(check_reference_dirs(&many, own.as_path()).is_err());
        assert_eq!(
            parse_reference_dirs(Some(&serde_json::to_string(&got).unwrap())),
            got
        );
        assert!(parse_reference_dirs(Some("not json")).is_empty());
        assert!(parse_reference_dirs(None).is_empty());
    }

    #[test]
    fn the_playbook_says_what_the_task_is_for() {
        let research =
            playbook_section(Some(TaskKind::Research), &["/r/dishorb".into()], "/files/p");
        assert!(research.contains("research task"));
        assert!(research.contains("alma-report"));
        assert!(research.contains("Do not call ExitPlanMode"));
        assert!(research.contains("/files/p"));
        assert!(research.contains("- `/r/dishorb`"));
        let plan = playbook_section(Some(TaskKind::Plan), &[], "/files/p");
        assert!(plan.contains("alma-plan") && !plan.contains("Reference folders"));
        assert!(playbook_section(None, &[], "/files/p").is_empty());
    }
}
