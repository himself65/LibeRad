import {
  GraphQLObjectType, GraphQLString,
  GraphQLID, GraphQLInt, GraphQLSchema,
  GraphQLNonNull
} from 'graphql'
import User from '../schema/user'

const UserType = new GraphQLObjectType({
  name: 'User',
  fields: () => ({
    id: { type: GraphQLID },
    name: { type: GraphQLString },
    age: { type: GraphQLInt }
  })
})

const RootQuery = new GraphQLObjectType({
  name: 'RootQueryType',
  fields: {
    user: {
      type: UserType,
      args: {
        id: {
          type: GraphQLID
        }
      },
      resolve: (parent, args) => User.findById(args.id)
    }
  }
})

const Mutation = new GraphQLObjectType({
  name: 'Mutation',
  fields: {
    addUser: {
      type: UserType,
      args: {
        name: {
          type: new GraphQLNonNull(GraphQLString)
        },
        age: {
          type: new GraphQLNonNull(GraphQLInt)
        }
      },
      resolve: (parent, args) => {
        return new User({
          name: args.name,
          age: args.age
        }).save()
      }
    },
    editUser: {
      type: UserType,
      args: {
        id: {
          type: new GraphQLNonNull(GraphQLID)
        },
        name: {
          type: GraphQLString
        },
        age: {
          type: GraphQLInt
        }
      },
      resolve: (parent, args) => {
        const id = args.id
        const data = {
          name: args.name,
          age: args.age
        }
        User.findByIdAndUpdate(id, data)
      }
    }
  }
})

export default new GraphQLSchema({
  query: RootQuery,
  mutation: Mutation
})
