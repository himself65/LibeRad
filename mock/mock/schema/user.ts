import { Schema, model } from 'mongoose'

const userSchema = new Schema({
  name: String,
  age: Number
})

export default model('User', userSchema)
