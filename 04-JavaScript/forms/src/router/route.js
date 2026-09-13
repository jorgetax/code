import {Router} from 'express'

const router = Router()

router.get('/hello', (req, res) => {
  res.json({dat: "Hello World!"})
})

export default router