import { useState, useEffect } from "react"
import { MessageList } from "./MessageList"
import { MessageInput } from "./MessageInput"
import type { MessageData } from "./Message"
import { sendToClaudeCode, checkClaudeInstalled } from "@/hooks/useClaudeCode"

export function Chat() {
  const [messages, setMessages] = useState<MessageData[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [status, setStatus] = useState<string | null>(null)
  const [claudeInstalled, setClaudeInstalled] = useState<boolean | null>(null)

  useEffect(() => {
    checkClaudeInstalled().then(setClaudeInstalled)
  }, [])

  const handleSend = async (content: string) => {
    const userMessage: MessageData = {
      id: crypto.randomUUID(),
      role: "user",
      content,
      timestamp: new Date(),
    }

    setMessages((prev) => [...prev, userMessage])
    setIsLoading(true)
    setStatus("Thinking...")

    try {
      const response = await sendToClaudeCode(content, {
        onStatus: (newStatus) => {
          setStatus(newStatus)
        },
      })

      const assistantMessage: MessageData = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: response || "No response received",
        timestamp: new Date(),
      }

      setMessages((prev) => [...prev, assistantMessage])
    } catch (error) {
      console.error("Failed to send message:", error)
      const errorMessage: MessageData = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: `Error: ${error instanceof Error ? error.message : String(error)}`,
        timestamp: new Date(),
      }
      setMessages((prev) => [...prev, errorMessage])
    } finally {
      setIsLoading(false)
      setStatus(null)
    }
  }

  return (
    <div className="flex h-screen flex-col bg-background">
      <header className="border-b px-4 py-3">
        <div className="flex items-center justify-between">
          <h1 className="text-lg font-semibold">Claude Personal Assistant</h1>
          {claudeInstalled !== null && (
            <span
              className={`text-xs ${claudeInstalled ? "text-green-600" : "text-red-600"}`}
            >
              {claudeInstalled ? "Claude Code: Connected" : "Claude Code: Not found"}
            </span>
          )}
        </div>
      </header>
      <MessageList messages={messages} isLoading={isLoading} status={status} />
      <MessageInput onSend={handleSend} disabled={isLoading || claudeInstalled === false} />
    </div>
  )
}
