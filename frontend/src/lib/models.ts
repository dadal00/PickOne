export enum Color {
  Red = 'red',
  Blue = 'blue',
  Green = 'green',
  Purple = 'purple',
}

export type ChartData = {
  color: Color
  count: number
}

export type ColorValue = {
  label: string
  hex: string
}

export type ColorDictionary = Record<Color, ColorValue>

export type ClickAnimations = Array<{
  id: number
  x: number
  y: number
}>

export interface WebSocketState {
  total: number
  total_users: number
  red: number
  green: number
  blue: number
  purple: number
}
