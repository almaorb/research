# A research task has a kind; a report never goes through a plan

Lesson (2026-10-09): a go-to-market research task opened from the Alma IDE
started in plan mode, loaded `alma-plan`, could not write its report (plan
mode denies every edit), and ended in an engineering plan the supervisor then
"built" phase by phase with shell checks that counted URLs. Research and
planning are different jobs.

- A session carries `task_kind` (`src/local/task_kind.rs`): `research` runs in
  auto mode with the `research-report` skill and its artifacts folder as an
  `--add-dir`, never a plan card; `plan` runs in plan mode with `alma-research`
  and `alma-plan`. No kind is the old behaviour.
- Pick the kind when the session is created (`kind` on `POST /api/chat/sessions`,
  the composer's task picker, `research_new` in the Alma MCP). Do not infer it
  from the permission mode alone.
- A session may read other repositories as `reference_dirs`; Claude Code gets
  them as `--add-dir` with edits denied (`read_only_rules` in `src/local/claude.rs`).
- Claude's permission ids are `manual`, `acceptEdits`, `plan`, `auto`,
  `bypassPermissions`; `default` is OpenCode's, not Claude's.
