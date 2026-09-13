import express from 'express'
import cors from 'cors'
import path from 'path'
import {fileURLToPath} from 'url'
import fsRouter from './lib/fs-router.js'

const app = express()
const port = 3000

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

app.use(cors())
app.use(express.json())

app.use(express.static(path.join(__dirname, '../public')))

app.use('/api', fsRouter)

app.use((req, res, next) => {
  if (req.path.startsWith('/api')) return res.json({message: 'Not Found'})
  return next()
})

app.use((req, res, next) => {
  return res.sendFile(path.join(__dirname, '../public/index.html'))
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})