---
name: writing-plans
description: Use when brainstorming is complete and you need to document technical design decisions before creating OpenSpec proposal
---

# Writing Plans

## Overview

Write comprehensive design documents for multi-step features. Document technical decisions, architecture, and trade-offs. This feeds directly into the OpenSpec planning workflow.

Assume the reader is a skilled developer but knows little about our toolset or problem domain.

**Announce at start:** "I'm using the writing-plans skill to create the implementation plan."

**Context:** This skill is called by brainstorming skill after design exploration is complete.

**Output:** Creates `openspec/changes/<change-id>/design.md` directly (no intermediate files)

**Change ID Selection:**
1. Generate change-id with date prefix: `YYYY-MM-DD-<description>` (e.g., "2026-03-06-add-dark-mode")
2. Ask user to confirm or provide alternative description
3. Check if `openspec/changes/<change-id>` already exists (should not with date prefix)
4. Check if `openspec/changes/archive/` contains same change-id (rare but possible)
5. Create directory: `mkdir -p openspec/changes/<change-id>`
6. Create design.md in that directory

## Bite-Sized Task Granularity

**Each step is one action (2-5 minutes):**
- "Write the failing test" - step
- "Run it to make sure it fails" - step
- "Implement the minimal code to make the test pass" - step
- "Run the tests and make sure they pass" - step
- "Commit" - step

## Design Document Structure

**Output to:** `openspec/changes/YYYY-MM-DD-<description>/design.md`

**Note**: This design.md documents technical decisions for THIS specific change. Long-term architectural patterns should go in `openspec/specs/[capability]/patterns.md` after archiving (if applicable).

**Document structure:**

```markdown
# [Feature Name] Design

## Context
[Background, constraints, stakeholders]

## Goals / Non-Goals
- Goals: [What we're solving]
- Non-Goals: [What we're explicitly not doing]

## Architecture
[2-3 sentences about technical approach]

## Tech Stack
[Key technologies/libraries/dependencies]

## Decisions
- Decision: [What and why]
- Alternatives considered: [Options + rationale]

## Risks / Trade-offs
- [Risk] → Mitigation

## Migration Plan
[Steps, rollback strategy if applicable]

## Open Questions
- [Any remaining uncertainties]

---
```

## Remember
- This design.md feeds into openspec-planning skill
- Focus on technical decisions and architecture
- Document alternatives considered and trade-offs
- Keep it concise but complete
- After saving, automatically invoke openspec-planning skill

## Handoff to OpenSpec

After saving design.md, automatically invoke openspec-planning skill:

**"Design document saved to `openspec/changes/<change-id>/design.md`. Now invoking openspec-planning to create the complete proposal."**

Pass the change-id to openspec-planning so it knows which directory to work with.

The openspec-planning skill will:
1. Read the design.md you just created from `openspec/changes/<change-id>/`
2. Create proposal.md (Why/What/Impact)
3. Create specs/<capability>/spec.md (Requirements)
4. Create tasks.md (Implementation checklist)
5. Validate the complete change
6. Present to user for approval
