export enum Mode {
  AUTOMATIC,
  MANUAL
}

export enum Print {
  "B&W",
  COLOR
}

export enum Layout { A = "A", B = "B", C = "C" }

export interface Plan {
  title: string
  price: number
  copies: 1 | 2 | 3
  popular: boolean
}

export interface Addon {
  title: string,
  price: number,
  enabled: boolean
}

export interface LayoutData {
  kind: Layout,
  title: string,
  disclaimer: string,
  disabled: boolean
}

export interface Options {
  layout: Layout | null,
  copies: number | null,
  digital: boolean,
  print: Print | null
}