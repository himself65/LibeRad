import mongoose from 'mongoose'
import { globalLogger } from './util'

export const mongoDBUrl = 'mongodb://localhost:27017'

export default async function () {
  const db = mongoose.connection
  db.on('error', err => globalLogger('Connection error.\n', err))
  db.once('open', () => globalLogger('MongoDB connected.'))
  return mongoose.connect(mongoDBUrl, {
    useNewUrlParser: true,
    useUnifiedTopology: true,
    auth: {
      user: 'root',
      password: '123456'
    }
  }).then(client => {
    return client
  }).catch(err => globalLogger(`Connect to ${mongoDBUrl} error.\n`, err))
}
