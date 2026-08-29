export enum Mode {
  AUTOMATIC = "Automatic",
  MANUAL = "Manual"
}

export enum Filter {
  BW = "B&W",
  Color = "Color"
}

export enum Layout {
  Full1x2 = "Full1x2",
  Full2x2 = "Full2x2",
  Strip1x3 = "Strip1x3",
  Strip1x4 = "Strip1x4",
}

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
  copies: number | null,
  digital: boolean,
  layout: Layout | null,
  filter: Filter | null,
}