import { invoke } from '@tauri-apps/api'
import { Layout, Spin, message } from 'antd'
import React, { lazy, memo, useCallback, useEffect, useState } from 'react'
import { useParams, useSearchParams } from 'react-router-dom'
import { readConfig } from '~/lib/settings'
import '~/styles/chat.scss'

const ChatContent = lazy(() => import('~/components/Content'))
const MessageInput = lazy(
  async () => await import('~/components/message/Input')
)

const { Content, Slider } = Layout

const getChatID = (): number => {
  const { chatID } = useParams<'chatID'>()

  return parseInt(chatID)
}

const Chat = memo(() => {
  const [waiting, setWaiting] = useState(false)
  const [retry, setRetry] = useState(false)
  const [showChatList, setShowChatList] = useState(false)
  const [messages, setMessages] = useState<Message[]>([])
  const [settings, setSettings] = useState<GlobalSettings | null>(null)

  const chatID = getChatID()

  const [searchParams] = useSearchParams()
  const chatTopic = searchParams.get('name') ?? '未知主题名'

  useEffect(() => {
    const fetchSettings = async (): Promise<void> => {
      const settings = await readConfig()

      setSettings(settings)

      if (!settings || settings.authKey === '') {
        // TODO: open global settings page
      }
    }

    void fetchSettings()
  }, [])

  const onSendPrompt = useCallback(
    async (prompt: string): Promise<void> => {
      // const createAt = newMessage(content)

      // setWaiting(true)
      // setRetry(false)

      console.log('sending prompt', prompt)

      const proxyConfig: ProxyConfig = {
        method: 'proxy',
        proxy: {
          protocol: 'socks5h',
          host: '127.0.0.1',
          port: 1086,
        },
      }

      const psid = ''

      let snlm0e: string

      try {
        snlm0e = await invoke<string>('get_snlm0e', {
          proxyConfig,
          psid,
        })
      } catch (e) {
        void message.error(String(e))

        return
      }

      const requestId = 243279

      try {
        await invoke('get_answer', {
          proxyConfig,
          snlm0e,
          prompt,
          requestId,
          psid,
          conversationId: '',
          responseId: '',
          choiceId: '',
        })
      } catch (e) {
        void message.error(String(e))

        return
      }
    },
    [messages]
  )

  if (settings === null) {
    return (
      <Spin tip="reading settings">
        <div className="content" />
      </Spin>
    )
  }

  return (
    <Content>
      <React.Suspense fallback={null}>
        <ChatContent
          messages={messages}
          settings={settings}
          showChatList={showChatList}
          chatTopic={chatTopic}
        />
      </React.Suspense>

      <React.Suspense fallback={null}>
        <MessageInput
          chatID={chatID}
          onSendPrompt={onSendPrompt}
          resetMessageList={() => {
            setMessages([])
          }}
          waiting={waiting}
          retry={retry}
        />
      </React.Suspense>
    </Content>
  )
})

Chat.displayName = 'Chat'

export default Chat
