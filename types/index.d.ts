export interface User {
  name: string
  age: number
  gender: 0 | 1
  links: {
    name: string
    to: string
  }[]
}
