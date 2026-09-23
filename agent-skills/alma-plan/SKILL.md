---
name: alma-plan
description: "Write an engineering plan the Alma IDE's supervisor can run unattended: ordered phases, each with a brief and at least one shell check that decides on its own whether the phase worked. Use in plan mode, once the research is done and the human has heard the shape, before calling ExitPlanMode."
---

# A plan the supervisor can run

When the human approves your plan with **Approve and build**, the Alma IDE's
supervisor takes over and you are done. It hands each phase, as a brief, to a
builder: Claude Code in one of the editor's terminals, in a worktree of the
phase's own, starting cold with only the brief, the design records and the
run branch to go on. When the builder stops, the supervisor runs that phase's
checks; the checks alone decide whether the phase passed, and a failure goes
back to the builder with its output. Nobody is watching in between. So the
plan you present must be one a machine can drive, and every brief must stand
on its own.

## The contract

Your `ExitPlanMode` plan is Markdown for the human — goal, what was found,
the approach and why, the phases in prose — and it ENDS with exactly one
fenced `json` block in this shape:

```json
{
  "title": "a short name for the run",
  "phases": [
    {
      "title": "what this phase delivers",
      "intent": "the brief the builder is handed: what to do, and what done looks like",
      "checks": [
        {"description": "VERIFY (passes today): the workspace builds", "command": "cargo build -p app", "expect": "success"},
        {"description": "BUILD: /api/invoices answers with the new list", "command": "cargo test -p app invoices_list", "expect": "success"}
      ]
    }
  ]
}
```

- Every phase needs at least one check. A check is a shell command that runs
  in the repository and decides, on its own, whether the phase worked. No
  phase may end in a person looking at something. If a phase cannot be
  checked that way, split it or rewrite it until it can.
- `expect` is one of `"success"` (exits zero, the default), `"exit:N"`,
  `"contains:TEXT"`, `"excludes:TEXT"`.
- **Every check is VERIFY or BUILD, and the editor tries them all before it
  accepts the plan**, on this worktree as it stands:
  - A check whose description starts with `VERIFY` pins something that is
    true today and must stay true: the build, the existing tests, a route
    that already answers. It must **pass** today, or the plan comes back.
  - Every other check is a BUILD check: the gate the phase's work is aimed
    at. Start its description with `BUILD:`. It must **fail** today, or the
    plan comes back — a gate that is already open cannot tell the work from
    its absence, and the phase would pass with nothing done. Point it at
    what the work creates: `cargo test new_test_name`, `test -s NEW_FILE`,
    `grep -q 'fn new_route' src/routes.rs`, a URL that 404s today.
  - Every phase needs at least one BUILD check. Prefer checks that already
    exist for VERIFY (the project's tests, its build, its linter).
  - `$ALMA_PORT` is the run's own port, set in every check and in the
    builder's shell; a service the plan starts listens there.
- Order the phases so each one leaves the tree working. Do not number them;
  order is position in the list. Four to eight phases is usually right; a
  phase is an hour or two of work.
- A check may probe a URL (`{"description": ..., "http": "http://127.0.0.1:3000/health",
  "expect_status": 200}`) or a page in the editor's own browser
  (`{"description": ..., "browser": "<url>", "script": "<javascript that
  evaluates to a truthy value once the page is right>"}`). **A phase that
  adds or changes anything a person sees in a browser must carry a browser
  check.** A build passing says nothing about a page: one that served its
  own source as text, and one that froze the editor on open, both passed
  every shell check they had.
- A phase that writes a script must run it, not parse it. `bash -n` and
  `--help` prove nothing; if the script cannot run against the instance
  that is verifying the phase, give it a dry-run mode and check that, or
  run it against a second instance on another port.
- Optional top-level keys: `"policy"` (standing rules of the environment,
  quoted into every brief; without it the default says everything is
  preinstalled and nothing may be installed, containerised or scaffolded),
  `"executor": {"ssh": {"host": ..., "directory": ...}}` to build on a
  remote host over ssh and tmux, `"references"` (a list of absolute paths
  or clone URLs of the projects the plan builds on — what `catalog_match`
  found — which the supervisor fetches before the first phase and names in
  every brief, so each builder reads them directly by path; nothing is
  indexed), and per phase `"id"` and `"depends_on"`.
- Before `ExitPlanMode`, write the design down as files in the worktree so
  the supervisor and every phase can read them: `docs/inventory.md` (what
  already exists and where — the capability map from the research),
  `docs/whitepaper.md` (the problem, prior art, the approach and why) and
  one `docs/adr/NNNN-<slug>.md` per significant choice (context, options
  considered, decision, consequences). Every brief names them.
- The prose above the block and the block must agree. The block is what
  runs.

If the plan comes back with "That plan cannot be run by the Alma supervisor:
…", fix exactly what it names and present the plan again. A BUILD check that
"would pass before its work exists" needs a sharper target, not a VERIFY
label: relabel it only if it really is meant to hold before the work.
