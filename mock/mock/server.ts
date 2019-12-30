import express from 'express'
import morgan from 'morgan'
import bodyParser from 'body-parser'
import connectDB from './db'
import graphqlHTTP from 'express-graphql'
import schema from './graphql'

(async () => {
  const app = express()

  app.use(morgan('combined'))
  app.use(bodyParser.json())

  app.all('*', function (req, res, next) {
    res.header('Access-Control-Allow-Origin', '*')
    res.header('Access-Control-Allow-Headers',
      'Content-Type, Content-Length, Authorization, Accept, X-Requested-With , yourHeaderFeild')
    res.header('Access-Control-Allow-Methods', 'PUT, POST, GET, DELETE, OPTIONS')

    if (req.method === 'OPTIONS') {
      res.send(200)
    } else {
      next()
    }
  })

  await connectDB()

  app.use('/graphql', graphqlHTTP({
    schema
  }))

  app.listen(3001, () => {
    console.log('listen at http://localhost:3001')
  })
})()
