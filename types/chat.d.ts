type ChatProps = Omit<MessageListProps, 'showLineNumbers'> & {
  chatTopic: string
  settings: GlobalSettings
}
