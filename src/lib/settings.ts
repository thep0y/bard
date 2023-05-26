import { invoke } from '@tauri-apps/api'
import { message } from 'antd'

export const defaultSettings: GlobalSettings = {
  proxy: undefined,
  authKey: '',
  showLineNumbers: false,
}

interface SettingsStruct {
  proxy: {
    method: ProxyMethod
    proxy?: Proxy
    reverse_proxy?: ReverseProxy
  }
  auth_key: string
  show_line_numbers: boolean
}

export const PROTOCOLS = [
  [
    {
      value: 'http',
      label: 'http',
    },
    {
      value: 'https',
      label: 'https',
    },
    {
      value: 'socks5',
      label: 'socks5',
    },
    {
      value: 'socks5h',
      label: 'socks5h',
    },
  ],
]

export const readConfig = async (): Promise<GlobalSettings> => {
  try {
    const settings = await invoke<SettingsStruct>('read_config')

    if (settings === null || settings.auth_key === '') {
      await message.warning(
        'Configuration file does not exist or is missing key fields.'
      )
    }

    return {
      proxy: {
        method: settings?.proxy?.method ?? 'direct',
        proxy: settings?.proxy?.proxy,
        reverseProxy: settings?.proxy?.reverse_proxy,
      },
      authKey: settings?.auth_key ?? '',
      showLineNumbers: settings?.show_line_numbers ?? false,
    }
  } catch (e) {
    void message.error(String(e))

    return defaultSettings
  }
}
