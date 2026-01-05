import { invoke, Channel } from "@tauri-apps/api/core"

interface StreamEvent {
  event: string  // "status", "chunk", "done"
  data: string
}

interface ClaudeResponse {
  result: string
  session_id: string | null
}

export interface SendMessageOptions {
  cwd?: string
  continueSession?: boolean
  onStatus?: (status: string) => void
  onChunk?: (chunk: string) => void
}

export async function checkClaudeInstalled(): Promise<boolean> {
  try {
    return await invoke<boolean>("check_claude_installed")
  } catch {
    return false
  }
}

export async function clearSession(): Promise<void> {
  await invoke("clear_session")
}

export async function sendToClaudeCode(
  message: string,
  options: SendMessageOptions = {}
): Promise<string> {
  const { cwd, continueSession = true, onStatus, onChunk } = options

  const onEvent = new Channel<StreamEvent>()

  onEvent.onmessage = (event: StreamEvent) => {
    if (event.event === "status" && onStatus) {
      onStatus(event.data)
    } else if (event.event === "chunk" && onChunk) {
      onChunk(event.data)
    }
  }

  const response = await invoke<ClaudeResponse>("send_to_claude", {
    message,
    cwd: cwd ?? null,
    continueSession,
    onEvent,
  })

  return response.result
}
