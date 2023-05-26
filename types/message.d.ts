interface Message {
  role: Role
  content: string
  createAt: number
}

interface MessageInputProps {
  onSendPrompt: (prompt: string) => Promise<void>
  resetMessageList: () => void
  waiting: boolean
  retry: boolean
  chatID: number
}

interface MessageListProps {
  messages: Message[]
  showChatList: boolean
  showLineNumbers: boolean
}
