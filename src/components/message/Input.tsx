import { invoke } from '@tauri-apps/api/tauri'
import React, { memo, useCallback, useEffect, useState } from 'react'
import { Affix, Button, ButtonProps, Space, Tooltip, message } from 'antd'
import { ClearOutlined, LoadingOutlined, SendOutlined } from '@ant-design/icons'
import TextArea from 'antd/es/input/TextArea'

const MessageInput = memo(
  ({
    chatID,
    onSendPrompt,
    resetMessageList,
    waiting,
    retry,
  }: MessageInputProps) => {
    const [chatMessage, SetChatMessage] = useState('')

    const onEnter = useCallback((): void => {
      const msg = chatMessage.trim()

      if (msg === '') {
        SetChatMessage(msg)

        return
      }

      void onSendPrompt(msg)

      SetChatMessage('')
    }, [chatMessage, onSendPrompt])

    const clearMessages = useCallback(async (): Promise<void> => {
      try {
        await invoke('clear_chat', { chatId: chatID })

        resetMessageList()
      } catch (e) {
        void message.error(String(e))
      }
    }, [chatID])

    const onChange = useCallback(
      (e: React.ChangeEvent<HTMLTextAreaElement>): void => {
        SetChatMessage(e.target.value)
      },
      [chatMessage]
    )

    const onKeyDown = (e: React.KeyboardEvent) => {
      if (e.key === 'Enter' && e.shiftKey) {
        e.preventDefault()

        SetChatMessage((pre) => pre + '\n')

        return
      }

      if (e.key === 'Enter') {
        e.preventDefault()

        void onEnter()
      }
    }

    const statusButton = () => {
      const disabled = waiting || chatMessage.trim() === ''
      const icon = waiting ? <LoadingOutlined /> : <SendOutlined />

      const commonButtons: ButtonProps = {
        type: 'primary',
        onClick: onEnter,
        disabled,
        danger: waiting,
      }

      return <Button {...commonButtons}>{icon}</Button>
    }

    return (
      <div id="input">
        <Affix style={{ width: '90%', maxWidth: 800 }}>
          <Space.Compact block style={{ alignItems: 'end' }}>
            <Tooltip title="Clear the chat">
              <Button type="primary" disabled={waiting} onClick={clearMessages}>
                <ClearOutlined />
              </Button>
            </Tooltip>

            <TextArea
              value={chatMessage}
              placeholder="Input the prompt you want to send"
              onChange={onChange}
              onKeyDown={onKeyDown}
              maxLength={2000}
              autoSize={{ minRows: 1, maxRows: 10 }}
              style={{ borderRadius: 0 }}
              showCount
              allowClear
            />

            {statusButton()}
          </Space.Compact>
        </Affix>
      </div>
    )
  }
)

MessageInput.displayName = 'MessageInput'

export default MessageInput
