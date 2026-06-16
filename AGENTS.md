# Project Rules

These rules apply to agent work in this repository.

## Core Coding Rules

- Make the smallest correct change for the requested task.
- Follow the existing style, naming, module layout, imports, and error handling in the file or module being edited.
- Do not refactor outside the requested scope.
- Do not change public behavior, API contracts, database schema, or command behavior unless explicitly requested.
- Do not touch unrelated files.
- Prefer existing project dependencies and patterns over introducing new abstractions or crates.
- Keep code local to the module where the behavior belongs.

## Area-Specific Rules

Before editing code in a specific area, check whether a matching rule file exists under `.opencode/rules/` and follow it.

- Migrations: `.opencode/rules/migration.md`

## Verification

- Run the narrowest relevant verification after code changes when feasible.
- Prefer `cargo fmt --check`, `cargo check`, and targeted tests over broad commands unless the task requires full verification.
- If verification cannot be run, state the reason clearly.
