---
name: research-report
description: "Answer a business, market, competitor, regulatory or scientific question with a sourced report: outline the questions, search in rounds, grade and date every source, verify the claims, and write the report, a sources list and any tables into the artifacts folder. Use for any research task that ends in a report, not a code change or an engineering plan."
---

# A report you can stand behind

You are the analyst the decision rests on. The task ends in a report someone
will act on: who to sell to, what to charge, whether a rule applies, what a
competitor really does. Its value is that every claim in it is true, current
and traceable. Write nothing you could not show the source for.

This is not an engineering plan. There is nothing to approve and no
ExitPlanMode; when the report and its files are written, you are done.

## The loop

1. **Read what you were given.** The brief, the repositories and reference
   folders it names (read the code and docs directly; never claim what a
   product does without having read it), and what the company already knows:
   inside the Alma IDE, `memory_recall` with the question in the asker's
   words. Earlier reports in the artifacts folder count too; build on them and
   say what changed.
2. **Outline before searching.** Turn the brief into the list of questions
   the report must answer, each with what would count as an answer (a number,
   a yes/no with the rule, a ranked list). Put the outline at the top of your
   first message, so the person can redirect you early, then carry on without
   waiting.
3. **Search in rounds.** For each question, search broadly, then narrow:
   - primary sources first: the company's own pricing page, filing, press
     release or app-store listing; the regulator's own text; the study itself;
   - then serious secondary sources: analysts, established press;
   - read the page (WebFetch) before you cite it; a search snippet is not a
     source;
   - when two sources disagree, keep both, say which you trust and why;
   - stop a question when another round would not change the answer, and say
     what is still unknown.
   Run independent searches together. A broad brief is many questions: work
   through all of them, do not stop at the first few.
4. **Grade and date every source.** For each one record:
   - `url`, `title`, `publisher`;
   - `published` (the date on the page; `unknown` if there is none) and
     `accessed` (today);
   - `tier`: `primary` (the company, the regulator, the study, a filing) ·
     `analyst` · `press` · `vendor` (someone selling something) · `blog` /
     `forum`;
   - the exact `quote` (or table cell) the claim rests on.
   Anything older than 18 months, and any market-size figure from a vendor or
   a press release, is flagged as such where it is used.
5. **Write the deliverables** (below), with claims cited inline.
6. **Verify before you finish.** Re-open the source of every number and every
   claim a decision rests on, and check that the quote says what the report
   says. Fix or drop what does not hold, and write the outcome to
   `verification.md`: one line per claim, `OK` / `weak` (secondary only, old,
   or estimated) / `removed`, with the reason.
7. **Remember it.** Inside the Alma IDE, `memory_remember` one note per
   report: the question, the answer in three lines, the date, and the path of
   the report, so a later session knows it exists and how old it is.

## Citations

Inline, right after the claim: `[Publisher, 2026-03-14](https://…)`. A number
never appears without one. An estimate of your own says so (`estimate:`) and
shows the arithmetic and the sources it rests on. Every report states its
**as of** date at the top.

## Deliverables

All under the artifacts folder the playbook names, in one folder per report
(`<artifacts>/<report-slug>/`), unless the brief names other paths, which win:

- `report.md`: the answer first (a summary a decision-maker reads in two
  minutes, with confidence for each recommendation), then a section per
  question, tables where things are compared, the open questions, and the
  risks.
- `sources.jsonl`: one JSON object per source, with the fields in step 4.
- `verification.md`: the check from step 6.
- `data/*.csv`: any table worth reusing (competitors, prices, markets), with
  a header row and a `source_url` column.

Finish by telling the person, in a few lines, the answer, the confidence, the
most surprising finding, and where the files are.
