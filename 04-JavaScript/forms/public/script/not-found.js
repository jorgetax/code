import Layout from './layout.js'
import {navigate} from "./router.js"

export default function PageNotFoud() {
  const content = `
      <section>
          <h1>Not Found</h1>
          <button></button>
      </section>
  `

  return Layout(content, "Not Found")
}