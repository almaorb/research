---
name: alma-plan
description: "Write the PLAN.md checklist the Alma factory runs: 5 to 10 broad phases, each with check: lines that decide on their own whether the phase worked. Use once the research is done and the human has heard the shape, before presenting the plan. Never for a research report (that is research-report)."
---

# A plan the factory can run

The Alma factory runs a plan as **one Claude Code session** in a terminal the
person can watch, in a git worktree of the run's own. The supervisor types
one line per phase ("do phase 3 of PLAN.md"); the builder builds, commits
and stops; the supervisor runs that phase's `check:` lines and types either
the next phase or the output of the checks that failed. The checks alone
decide whether a phase passed. Nobody is watching in between, so every
phase must end in something a machine can check.

## The checklist

The plan is a Markdown file, `PLAN.md`, at the root of the repository the run
is cut from. Write it there, or end your answer with it whole:

```markdown
# Booking app

A booking page for the clinic, on the existing API. (The goal, in words.)

references: /Users/marco/code/alma-meet
budget: 8h

- [ ] 1. Bookings API
  Add the bookings table and the REST routes, with tests.
  check: the API tests pass `cargo test -p bookings`
- [ ] 2. Booking page
  The /book page: a form that creates a booking and shows it.
  check: http http://127.0.0.1:$ALMA_PORT/book
  check: browser http://127.0.0.1:$ALMA_PORT/book :: !!document.querySelector('form')
```

- **5 to 10 broad phases** (at most 12). A phase is a coherent piece of the
  product, an hour or more of work, not a step. The session carries its
  context from phase to phase, so do not repeat earlier phases' details.
- Each phase is a top-level `- [ ] N. Title`, with what to build on the
  indented lines under it.
- **Every phase has at least one `check:` line.** A check is:
  - a shell command run in the run's worktree, passing when it exits zero;
  - `http <url> [status]`, passing on that status (200 when unsaid);
  - `browser <url> :: <javascript>`, run in Alma's own browser, passing when
    it evaluates to something truthy;
  - `device <ios|android> <app> :: <steps as a JSON array>`.

  A check may name itself in words with the command in backticks:
  ``check: the API tests pass `cargo test -p bookings` ``.
- A check must fail before its phase's work exists and pass once it does:
  point it at what the work creates (a new test by name, a route that 404s
  today, a file the phase writes). A check that already passes cannot tell
  the work from its absence.
- A check must not run or read a file the phase is meant to change; check
  the behaviour, not the script that tests it.
- A phase that changes anything a person sees in a browser carries a
  `browser` check: a build that passes says nothing about a page.
- `$ALMA_PORT` is the run's own port, in every check and in the builder's
  shell; anything the checks probe listens there.
- Optional lines before the phases: `references:` (absolute paths or clone
  URLs of projects the builder reads from, what `catalog_match` found),
  `budget: 8h`, `attempts: 3`, `policy:` (the environment's standing rules;
  the default is that everything is installed and nothing may be added).

## Handing it over

The person starts the run: **Run PLAN.md** on the supervisor strip, or
`plan_accept` with the checklist as `plan` and the repository as
`repository`. A plan that cannot run comes back with every problem at once;
fix exactly what it names.
