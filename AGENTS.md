# AI Agent Guide

## Project Identity & Purpose

**Dogfish** is an exploratory research project focused on *Conquest of Skyrim*, a Skyrim Special Edition mod by galvatron304 ([Nexus Mods #65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628)).

The project's purpose is to systematically investigate and understand the mod's strategic mechanics (faction management, territorial conquest, warfare, politics, and economy) and technical characteristics. This is strictly an exploratory research workspace, not an implementation project.

## Repository Structure

```text
├── AGENTS.md               # AI agent entry point and instructions
├── README.md               # Project overview and status
├── guardrails.md           # Research boundaries and rules (placeholder)
├── .gitignore              # Git ignore rules
├── research/
│   ├── findings/           # Consolidated, evidence-backed conclusions
│   ├── investigations/     # Technical research topics and questions
│   └── playtesting/        # Human gameplay observations and bug reports
├── references/
│   ├── README.md           # Overview of reference materials
│   └── sources/            # Reserved for future reference repositories/sources
└── docs/
    └── glossary.md         # Domain and engine terminology placeholder
```

## Organization of Research Material

Research is divided into two streams that feed into consolidated findings:

- **Technical Investigations (`research/investigations/`)**: Houses standalone technical research documents. Each investigation focuses on a specific question or subsystem (e.g., Papyrus scripting, dependencies, engine limits).
- **Playtesting Observations (`research/playtesting/`)**: Houses observations, reproduction steps for bugs, and empirical findings reported by human researchers during playtesting.
- **Consolidated Findings (`research/findings/`)**: Synthesizes verified conclusions supported by evidence from investigations or playtesting.
- **External References (`references/` and `references/sources/`)**: Stores source documentation, reference repositories, and external links.

## Agent Guidelines & Tooling

- **Commit Style**: Always use Conventional Commits (`docs:`, `chore:`, `feat:`, `fix:`, etc.) for any git commits.
- **Reverse Engineering**: Probing with REA (Reverse Engineer Anything / reverse engineering everything) is allowed for inspecting and analyzing binaries, scripts, assets, and system behavior.

## Future Agent Instructions

Operational instructions, specific workflows, and guidelines for AI agents will be maintained in this file (`AGENTS.md`) or expanded into dedicated documents under `docs/` as the project develops.
