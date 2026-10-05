# finding-027: the daily dashboard refresh grades items by a working convention the skill does not state, and the reasons behind each grade live only on one host

Date: 2026-10-05 (from the 2026-10-01 to 2026-10-05 scheduled refreshes)
Scope: skills/beadle-triage step 3 (classify), as practiced by the daily skills-method refresh of five boards
Status: observed over five runs; the convention is unratified

## What I saw

The daily refresh grades every new item into a priority lane before it lands on a
board. The skill tells the grader to re-classify on every axis, not to trust the
body's self-label, and to start low and escalate on evidence. In practice the grader
also used a narrower working rule, written nowhere but in its own run scripts:

1. Where a repository's issues carry a filer severity label, map low to P3, medium to
   P2 and high to P1 as the starting grade.
2. A lane definition overrides that starting grade. Two lanes did this across the five
   runs: P0a (silent data loss: state destroyed or stranded with no signal) and P0b
   (the tool reports a state that is not true). Both definitions come from the board
   fixtures, not from the skill.
3. Where an item resembled a known P0b by mechanism but no lane definition fit it
   exactly, the grader graded by analogy and wrote that down as its reason.

The reasons for each grade (why this item is P0b and not P2) were kept in per-run
tables inside the grader's scripts under a private notes directory on one host. The
board shows the grade and its chips, not the reason. Nobody has confirmed or overridden
the grades the rule produced; the 2026-10-04 harvest listed five of them as open.

## Why it matters

- Point 1 is in tension with the skill's "don't trust the body's self-label". A filer
  label is a self-label. Starting from it is defensible for repositories whose filers
  grade consistently, but it is a choice, and the skill does not make it.
- A second grader, or the same seat after a respawn, would not find the rule or the
  reasons, and could regrade the same items differently from run to run. That churn
  would show on public boards as unexplained priority moves.
- The lane definitions that override everything else (P0a, P0b) are defined only in a
  fixture document. A grader learns them by example.

## What would settle it

A ruling on point 1 (adopt it into step 3, or drop it), and a home for the P0a and P0b
definitions in the skill itself. Until then, the grading reasons stay private by design
(some boards are for private repositories), so a successor depends on that host's notes.

## Evidence

- Five refresh runs, 2026-10-01 to 2026-10-05, on five boards; every board gate passed.
- Grades given by analogy or by lane override in that window include a P0b by analogy
  with an earlier P0b, a P0a for mail that expires unread with no signal, and a P2 kept
  below P0b because polling still delivers.
- The per-run grade tables: private refresh notes on the host that ran the refresh
  (they include private-repository data and are not published).
