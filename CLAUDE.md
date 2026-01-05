# Claude Personal Assistant (CPA) - Development Guide

## Project Overview
A desktop app (Tauri + React) that wraps Claude Code CLI to make it accessible to non-technical users.

## Tech Stack
- **Frontend**: React + TypeScript + Tailwind + shadcn/ui
- **Backend**: Rust (Tauri)
- **LLM**: Claude Code CLI (subprocess)

## Development Commands

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build

# Run CLI integration tests
chmod +x scripts/test-claude-cli.sh && ./scripts/test-claude-cli.sh
```

## IMPORTANT: Testing Before Integration

**Always test CLI commands before integrating them into the Rust backend!**

Before adding any Claude Code CLI integration:

1. **Test the exact command in terminal first:**
   ```bash
   # Test the command you plan to use
   claude -p "test message" --output-format json
   ```

2. **Check for required flags:**
   - `--output-format stream-json` requires `--verbose` flag
   - `--output-format json` works without extra flags
   - `-p` (print mode) is required for non-interactive use

3. **Run the test script:**
   ```bash
   ./scripts/test-claude-cli.sh
   ```

4. **Common gotchas:**
   - `stream-json` format requires `--verbose` (discovered the hard way!)
   - Exit code 1 usually means wrong flags or missing requirements
   - Check stderr for error messages

## Project Structure

```
/claude-pa
├── src/                          # React frontend
│   ├── components/
│   │   ├── Chat.tsx              # Main chat container
│   │   ├── Message.tsx           # Individual message
│   │   ├── MessageList.tsx       # Scrollable message list
│   │   └── MessageInput.tsx      # Input with send button
│   └── hooks/
│       └── useClaudeCode.ts      # Tauri bridge to Claude CLI
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs                # Tauri setup
│   │   └── claude_bridge.rs      # Claude Code CLI spawning
│   └── Cargo.toml
├── scripts/
│   └── test-claude-cli.sh        # CLI integration tests
└── CLAUDE.md                     # This file
```

## Claude Code CLI Reference

```bash
# Non-interactive mode (required for app integration)
claude -p "prompt" --output-format json

# Available output formats:
# - text (default): Plain text response
# - json: Single JSON result with metadata
# - stream-json: Streaming JSON (requires --verbose)

# Useful flags:
# --allowedTools "Read,Edit,Bash"  # Auto-approve specific tools
# --cwd /path/to/dir               # Set working directory
# --continue                        # Continue last conversation
# --resume <session-id>            # Resume specific session
```

## Current Status
- [x] Basic chat UI
- [x] Claude Code CLI integration (json output)
- [x] Session continuity (--resume flag)
- [ ] Multi-assistant support
- [ ] Memory extraction (auto-update CLAUDE.md)
- [ ] Skill suggestions

## Session Management

The app maintains conversation continuity by:
1. Storing the `session_id` from each Claude response
2. Using `--resume <session_id>` for subsequent messages
3. This allows Claude to remember context from previous messages

To start a fresh conversation, use `clearSession()` from the frontend.

## Debugging

If Claude Code integration fails:
1. Check if Claude Code is installed: `claude --version`
2. Test the command directly: `claude -p "test" --output-format json`
3. Check the Tauri console for errors
4. Look at stderr in the Rust bridge
