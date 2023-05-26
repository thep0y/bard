declare type Protocol = 'http' | 'https' | 'socks5' | 'socks5h'

declare interface Proxy {
  protocol?: Protocol
  host?: string
  port?: number
}

declare type ReverseProxy = string

type ProxyMethod = 'direct' | 'proxy' | 'reverse-proxy'

interface GlobalSettings {
  proxy?: {
    method: ProxyMethod
    proxy?: Proxy
    reverseProxy?: ReverseProxy
  }
  authKey: string
  showLineNumbers: boolean
}
