import { get, writable } from 'svelte/store'
import { PUBLIC_WS_URL } from '$env/static/public'
import { visibility } from './visibility'
import { type WebSocketState } from '../models'

const MAX_RECONNECT_DELAY = 5000
export const connected = writable<boolean>(false)

export const websocket = (() => {
  const { subscribe, update } = writable<WebSocketState>({
    total: 0,
    total_users: 0,
    red: 0,
    green: 0,
    blue: 0,
    purple: 0,
  })

  let socket: WebSocket | null = null
  let reconnectTimer: NodeJS.Timeout | null = null

  const connect = () => {
    if (socket?.readyState === WebSocket.OPEN) return

    try {
      socket = new WebSocket(PUBLIC_WS_URL)
      socket.binaryType = 'arraybuffer'

      socket.onmessage = (event) => {
        const msg = JSON.parse(event.data)

        update((currentData) => ({
          ...currentData,
          ...msg,
        }))
      }
    } catch (e) {
      console.error('Network parse error:', e)
      return
    }

    socket.onopen = () => {
      console.log('connected')
      connected.set(true)
      if (reconnectTimer) {
        clearTimeout(reconnectTimer)
        reconnectTimer = null
      }
    }

    socket.onclose = () => {
      console.log('disconnected')
      connected.set(false)
    }

    socket.onerror = (e) => {
      console.error('connection error:', e)
      connected.set(false)
    }

    return socket
  }

  const attemptReconnect = () => {
    if (reconnectTimer) return
    console.log('reconnecting...')
    const delay = Math.min(Math.random() * 3000, MAX_RECONNECT_DELAY)
    reconnectTimer = setTimeout(() => {
      reconnectTimer = null
      connect()
      if (!get(connected) && get(visibility)) {
        attemptReconnect()
      }
    }, delay)
  }

  const sendPayload = (payload: string) => {
    if (socket?.readyState === WebSocket.OPEN) {
      socket.send('' + payload)
    } else {
      console.error('Cannot send vote: not connected')
      connected.set(false)
    }
  }

  const disconnect = () => {
    if (socket) {
      socket.close()
    }
  }

  return {
    subscribe,
    connect,
    sendPayload,
    disconnect,
    attemptReconnect,
  }
})()
