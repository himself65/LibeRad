use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(juniper::GraphQLEnum)]
pub enum Gender {
    Man = 0,
    Woman = 1,
}

#[derive(juniper::GraphQLObject)]
pub struct Link {
    pub name: String,
    pub to: String,
}

#[derive(juniper::GraphQLObject)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub gender: Gender,
    pub links: Vec<Link>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewLink {
    pub name: String,
    pub to: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub age: i32,
    pub gender: Gender,
    pub links: Vec<NewLink>,
}

pub struct QueryUser;

use juniper::FieldResult;
graphql_object!(QueryUser: () |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User {
            name: "himself65".to_string(),
            age: 18,
            gender: Gender::Woman,
            links: vec![
                Link {
                    name: "GitHub".to_string(),
                    to: "https://github.com/himself65".to_string(),
                },
                Link {
                name: "Twitter".to_string(),
                to: "https://twitter.com/himself_65".to_string(),
                }
            ],
        })
    }
});

pub struct MutationUser;

graphql_object!(MutationUser: () |&self| {
    field createUser(&executor, new_human: NewUser) -> FieldResult<User> {
        Ok(User {
            name: "himself66".to_string(),
            age: 19,
            gender: Gender::Man,
            links: vec![],
        })
    }
});

use juniper::RootNode;

pub type UserSchema = RootNode<'static, QueryUser, MutationUser>;

pub fn create_user_schema() -> UserSchema {
    UserSchema::new(QueryUser {}, MutationUser {})
}