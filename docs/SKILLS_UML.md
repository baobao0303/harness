# 🧠 Thư viện Kỹ năng (Skills Library) - UML Diagrams

## Overview

Harness cung cấp **34 skill** được tổ chức theo lifecycle phát triển phần mềm. Mỗi skill là một bộ hướng dẫn nghiệp vụ cụ thể giúp AI Agent thực hiện công việc theo quy trình chuẩn.

## Skill Lifecycle UML

```mermaid
sequenceDiagram
    participant Human
    participant Agent
    participant SkillLibrary
    participant CLI
    participant DB

    Human->>Agent: "Help me create a PRD"
    Agent->>SkillLibrary: Discover skill: harness-prd
    SkillLibrary-->>Agent: SKILL.md + description
    Agent->>CLI: harness skill run harness-prd
    CLI->>DB: Record trace
    DB-->>CLI: trace_id
    CLI-->>Agent: Execution result
    Agent-->>Human: PRD created + evidence
```

## Skill Categories UML

```mermaid
graph TD
    Root[Harness Skills Library] --> Init[Khởi Đầu]
    Root --> Requirements[Yêu Cầu]
    Root --> Architecture[Kiến Trúc]
    Root --> Planning[Lập Kế Hoạch]
    Root --> Design[Thiết Kế]
    Root --> Implementation[Triển Khai]
    Root --> Testing[Kiểm Thử]
    Root --> Review[Review]
    Root --> Docs[Tài Liệu]
    Root --> Special[Đặc Biệt]

    Init --> Help[harness-help]
    Init --> DocProject[harness-document-project]
    Init --> ProjContext[harness-generate-project-context]

    Requirements --> PRD[harness-prd]
    Requirements --> Brief[harness-product-brief]
    Requirements --> Elicitation[harness-advanced-elicitation]
    Requirements --> Brainstorm[harness-brainstorming]

    Architecture --> CreateArch[harness-create-architecture]
    Architecture --> TechResearch[harness-technical-research]

    Planning --> CreateEpics[harness-create-epics-and-stories]
    Planning --> CreateStory[harness-create-story]

    Design --> CreateUX[harness-create-ux-design]

    Implementation --> CheckReady[harness-check-implementation-readiness]
    Implementation --> CorrectCourse[harness-correct-course]

    Testing --> GenerateE2E[harness-qa-generate-e2e-tests]

    Review --> Retrospective[harness-retrospective]
    Review --> Checkpoint[harness-checkpoint-preview]

    Docs --> IndexDocs[harness-index-docs]
    Docs --> ShardDoc[harness-shard-doc]
    Docs --> Distillator[harness-distillator]

    Special --> PartyMode[harness-party-mode]
    Special --> Investigate[harness-investigate]
    Special --> Customize[harness-customize]
```

## IDE Integration UML

```mermaid
graph LR
    subgraph "Skill Source of Truth"
        Skills[.agents/skills/]
        SKILL_MD[SKILL.md per skill]
    end

    subgraph "IDE Discovery Files"
        Kiro[.kiro/steering/*.md]
        Cursor[.cursor/rules/*.mdc]
        Windsurf[.windsurfrules]
        AGENTS[AGENTS.md skill table]
    end

    subgraph "IDE Native UI"
        KiroUI[Kiro: Type # in chat]
        CursorUI[Cursor: Type @ or Rules panel]
        WindsurfUI[Windsurf: Auto-read]
        ClaudeUI[Claude Code: Read AGENTS.md]
        CopilotUI[GitHub Copilot: Read AGENTS.md]
    end

    Skills -->|install-ide-skills.sh| Kiro
    Skills -->|install-ide-skills.sh| Cursor
    Skills -->|install-ide-skills.sh| Windsurf
    Skills -->|install-ide-skills.sh| AGENTS

    Kiro --> KiroUI
    Cursor --> CursorUI
    Windsurf --> WindsurfUI
    AGENTS --> ClaudeUI
    AGENTS --> CopilotUI
```

## CLI Command Flow UML

```mermaid
sequenceDiagram
    participant User
    participant Shell
    participant RustCLI
    participant DB
    participant Skills

    User->>Shell: harness skill list
    Shell->>RustCLI: Parse args
    RustCLI->>DB: Query schema_version
    DB-->>RustCLI: version=2
    RustCLI->>Skills: Scan .agents/skills/
    Skills-->>RustCLI: List of 34 skills
    RustCLI-->>User: Display table (name, wrapper, description)

    User->>Shell: harness skill run harness-retrospective
    Shell->>RustCLI: Parse args
    RustCLI->>Skills: Find harness-retrospective
    Skills-->>RustCLI: Path + has_wrapper=true
    RustCLI->>Skills: Execute run.sh
    Skills-->>RustCLI: JSON result
    RustCLI-->>User: Print execution result
```

## Task Loop Integration UML

```mermaid
sequenceDiagram
    participant User
    participant Agent
    participant Intake
    participant Story
    participant Skill
    participant Trace
    participant Backlog

    User->>Agent: "I want to add a new feature"
    Agent->>Intake: Classify work (intake)
    Intake->>DB: INSERT INTO intake
    DB-->>Intake: intake_id=123
    
    alt normal/high-risk lane
        Agent->>Story: Create story packet (story add)
        Story->>DB: INSERT INTO story
        DB-->>Story: story_id=US-001
    end
    
    Agent->>Skill: Execute appropriate skill
    Skill->>DB: Record trace
    DB-->>Skill: trace_id=456
    
    alt friction found
        Agent->>Backlog: Record friction (backlog add)
        Backlog->>DB: INSERT INTO backlog
    end
    
    Agent->>Trace: Log completion (trace)
    Trace->>DB: INSERT INTO trace
    DB-->>Agent: trace_id=456
    
    Agent-->>User: Task completed + trace_id=456
```

## Skill Metadata Structure

```mermaid
classDiagram
    class Skill {
        +String name
        +String description
        +String path
        +Boolean has_wrapper
        +SKILL_MD skilLmd
    }

    class SKILL_MD {
        +String name
        +String description
        +String workflow
        +List~Step~ steps
    }

    class run_sh {
        +String shebang
        +String args
        +JSON output
    }

    Skill --> SKILL_MD : contains
    Skill --> run_sh : executes if has_wrapper=true
```

## IDE-Specific Discovery

| IDE | Discovery Mechanism | File Format | Trigger |
| :--- | :--- | :--- | :--- |
| **Kiro** | Steering files | `.kiro/steering/*.md` | Type `#` in chat |
| **Cursor** | Rule files | `.cursor/rules/*.mdc` | Type `@` or Rules panel |
| **Windsurf** | Consolidated file | `.windsurfrules` | Agent auto-read |
| **Claude Code** | AGENTS.md | Markdown table | Agent reads AGENTS.md |
| **GitHub Copilot** | AGENTS.md | Markdown table | Agent reads AGENTS.md |

## Skill Invocation Flow

```mermaid
flowchart TD
    A[User Request] --> B{Which IDE?}
    B -->|Kiro| C[Type # in chat]
    B -->|Cursor| D[Type @ or check rules]
    B -->|Windsurf| E[Agent auto-discover]
    B -->|Claude/Copilot| F[Read AGENTS.md]
    
    C --> G[Select skill from list]
    D --> G
    E --> G
    F --> G
    
    G --> H[Read .agents/skills/<name>/SKILL.md]
    H --> I{Has run.sh wrapper?}
    I -->|Yes| J[Execute run.sh]
    I -->|No| K[Follow workflow in SKILL.md]
    
    J --> L[Return JSON result]
    K --> L
    
    L --> M[Agent executes task]
    M --> N[Record trace in DB]
```
