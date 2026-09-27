# An existing folder already on GitHub syncs automatically

When a project is added from an existing folder whose `origin` is a GitHub
repository the user can push to, GitHub sync is on from the start. Marco
asked for it on 2026-09-27 ("when adding a new folder existing it should be
automatic"). Before that, sync defaulted to off. Projects such as alma,
alma-silicon, villanofyam and seataya showed "not connected" on
`alma://research/projects` even though their repository had been detected,
and each one had to be switched on by hand.

The rule has two halves, one on each side of the API:

- `ui/src/components/NewProjectForm.tsx` ticks the sync box once the repo
  access check says the user can push. If the user changes the box, their
  choice is kept.
- `create_project` in `src/commands/up.rs` applies the same rule when a caller
  (the MCP `research_project_create` tool, a script) does not send
  `githubSyncEnabled` at all.

Only a repository that already exists and can be pushed to turns sync on by
itself. Enabling sync for a folder without such a remote creates a new GitHub
repository, which stays an explicit choice (the box, or the Settings default
for new projects).
