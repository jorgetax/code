import {existsSync} from 'fs'
import {readdir} from 'fs/promises'
import {Router} from 'express'
import {pathToFileURL, fileURLToPath} from 'node:url'
import {dirname, join, sep} from 'node:path'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

const root = join(__dirname, '../router')
const idxFile = ['index', 'route']
const ext = ['.ts', '.js']

const router = Router()

if (!existsSync(root)) {
  throw new Error('Router directory not found')
}

async function dynamic(dir, base = '') {
  const entries = await readdir(dir, {withFileTypes: true})

  for (const entry of entries) {
    const pathname = join(dir, entry.name)

    if (entry.isDirectory()) {
      const next = base ? join(base, entry.name) : entry.name
      await dynamic(pathname, next)
      continue
    }

    if (!ext.some(e => entry.name.endsWith(e))) continue

    const filename = entry.name.replace(/\.(ts|js)$/, '')
    const path = idxFile.includes(filename) ? base || '/' : join(base, filename).replaceAll(sep, '/')

    const mod = await import(pathToFileURL(pathname).href)
    if (mod.default) router.use(path === '/' ? '/' : `/${path}`, mod.default)
  }
}

await dynamic(root)

export default router