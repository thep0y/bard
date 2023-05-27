declare type Protocol = 'http' | 'https' | 'socks5' | 'socks5h'

declare interface Proxy {
  protocol?: Protocol
  host?: string
  port?: number
}

declare type ReverseProxy = string

type ProxyMethod = 'direct' | 'proxy' | 'reverse-proxy'

interface ProxyConfig {
  method: ProxyMethod
  proxy?: Proxy
  reverseProxy?: ReverseProxy
}

interface GlobalSettings {
  proxy?: ProxyConfig
  authKey: string
  showLineNumbers: boolean
}
