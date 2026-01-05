# Claude Personal Assistant (CPA) - Architecture

## What We're Building

A **desktop app** that makes Claude Code accessible to non-technical users.

**Key insight**: Claude Code already has the hard stuff (tools, agent loop, MCP). What's missing is a friendly interface for managing context, memory, and skills.

```
┌─────────────────────────────────────────────────────────────┐
│                    CPA DESKTOP APP                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Friendly Chat UI                                           │
│  ├── Conversation interface (not terminal)                  │
│  ├── Assistant sidebar (switch contexts)                    │
│  └── Settings/preferences                                   │
│                                                             │
│  Our Value-Add Layer                                        │
│  ├── Memory Manager (auto-updates CLAUDE.md)                │
│  ├── Multi-Assistant (folder per context)                   │
│  └── Skill Helper (detects patterns, suggests skills)       │
│                                                             │
│  Claude Code CLI (subprocess)                               │
│  └── Full capabilities: files, bash, web, grep, MCP         │
│                                                             │
│  User's Local Files                                         │
└─────────────────────────────────────────────────────────────┘
```

---

## What We Build vs What Claude Code Provides

| We Build | Claude Code Provides |
|----------|---------------------|
| Friendly chat UI | Tools (file ops, bash, web, grep) |
| Memory manager (auto-extracts facts) | Agent loop (retries, tool chaining) |
| Multi-assistant management | MCP integrations |
| Skill suggestions | Skill infrastructure |
| Onboarding flow | CLAUDE.md context reading |

---

## Core Concept: The Assistant

Each assistant is a folder:

```
~/CPA/assistants/dads-health/
├── CLAUDE.md              # Auto-managed by us
├── .claude/
│   └── skills/            # Auto-generated
├── conversations/         # Local history
└── files/                 # User's related docs
```

Switching assistants = changing Claude Code's working directory.

---

## Memory Management

The core value-add. Users never edit CLAUDE.md manually.

### Flow
```
User sends message
       │
       ▼
Claude Code responds
       │
       ▼
Our layer intercepts
       │
       ▼
Extract facts (async Claude API call)
       │
       ▼
Append to CLAUDE.md
```

### CLAUDE.md Structure
```markdown
# Dad's Health Tracker

## About This Assistant
Helps track health information for user's father.

## Key Information
- Dad's name: John
- Conditions: Type 2 diabetes, heart disease
- Medications: Metformin, Lisinopril

## Remembered Facts
- Allergic to penicillin (2024-01-15)
- Next cardiology appointment: Feb 20 (2024-01-16)

## Preferences
- User prefers concise summaries
- Always remind about medication interactions
```

---

## Tech Stack

| Layer | Choice |
|-------|--------|
| Desktop Framework | Tauri |
| Frontend | React + TypeScript |
| Styling | Tailwind + shadcn/ui |
| Local Storage | SQLite |
| LLM | Claude Code CLI (subprocess) |
| State | Zustand |

### Why Tauri?
- Small app size (~10MB vs Electron's ~150MB)
- Rust backend for efficient subprocess management
- Better security model
- Web tech for UI

---

## Claude Code Integration

### Subprocess Approach
```rust
let mut child = Command::new("claude")
    .args(["--print"])
    .current_dir(assistant_folder)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
```

### Research Needed
- How does Claude Code handle streaming output?
- What CLI flags are available?
- Can we pipe messages to stdin?

---

## User Flow

1. **Install** desktop app
2. **Create** assistant ("I want help with my dad's health")
   - Our layer creates folder + initial CLAUDE.md
3. **Chat** with assistant
   - Full Claude Code capabilities
   - Memory auto-extracts and updates
4. **Switch** between assistants via sidebar
5. **Skill suggestions** appear for repeated patterns

---

## MVP Scope

### Included
- Desktop app running Claude Code
- Multi-assistant with separate folders
- Chat UI (markdown rendering)
- Auto-updating CLAUDE.md (memory)
- Conversation history

### Not Included (Post-MVP)
- Skill suggestions
- Onboarding wizard
- MCP configuration UI
- Cloud sync

---

## File Structure

```
/claude-pa
├── src/                        # React frontend
│   ├── App.tsx
│   ├── components/
│   │   ├── Chat.tsx
│   │   ├── MessageList.tsx
│   │   ├── MessageInput.tsx
│   │   ├── Message.tsx
│   │   └── Sidebar.tsx
│   ├── hooks/
│   │   └── useClaudeCode.ts
│   └── stores/
│       └── assistantStore.ts
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── claude_bridge.rs
│   │   └── assistant_manager.rs
│   └── Cargo.toml
└── package.json
```

---

## Success Criteria

1. Non-technical user can chat without terminal
2. Memory auto-updates across sessions
3. Multiple assistants with separate contexts
4. Seamless switching between assistants
5. Zero config file editing required
