export interface OhCodeOptions {
  origin: string[],
  modified: string[],
  parent: HTMLElement
}

export enum DiffType {
  Normal,
  Add,
  Delete,
}