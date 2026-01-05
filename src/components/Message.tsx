import { cn } from "@/lib/utils"

export interface MessageData {
  id: string
  role: "user" | "assistant"
  content: string
  timestamp: Date
}

interface MessageProps {
  message: MessageData
}

export function Message({ message }: MessageProps) {
  const isUser = message.role === "user"

  return (
    <div
      className={cn(
        "flex w-full",
        isUser ? "justify-end" : "justify-start"
      )}
    >
      <div
        className={cn(
          "max-w-[80%] rounded-lg px-4 py-2 text-sm",
          isUser
            ? "bg-primary text-primary-foreground"
            : "bg-muted text-muted-foreground"
        )}
      >
        <p className="whitespace-pre-wrap">{message.content}</p>
      </div>
    </div>
  )
}
