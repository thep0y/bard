import React, { lazy, memo, useCallback, useLayoutEffect, useRef } from 'react'

const MessageList = lazy(async () => await import('~/components/message/List'))

const Content = memo(({ messages, showChatList, settings }: ChatProps) => {
  const messageListComponentRef = useRef<HTMLDivElement>(null)
  const contentRef = useRef<HTMLDivElement>(null)

  const scrollToBottom = useCallback(() => {
    const current = contentRef.current

    current?.scrollTo({
      top: current.scrollHeight,
      behavior: 'smooth',
    })
  }, [])

  const observer = new ResizeObserver(scrollToBottom)

  useLayoutEffect(() => {
    const current = contentRef.current

    if (!current) return

    observer.observe(current)
  }, [messages])

  return (
    <div id="chat-content" ref={contentRef}>
      <div ref={messageListComponentRef}>
        <React.Suspense fallback={null}>
          <MessageList
            messages={messages}
            showChatList={showChatList}
            showLineNumbers={settings.showLineNumbers}
          />
        </React.Suspense>
      </div>
    </div>
  )
})

Content.displayName = 'ChatContent'

export default Content
