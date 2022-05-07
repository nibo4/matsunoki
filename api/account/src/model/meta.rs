use std::ops::Deref;

pub trait Identifier: Deref + PartialEq + Eq + Clone {}

pub trait Entity<T: Identifier> {
    fn id(&self) -> &T;
}

pub trait AggregateRoot<T: Identifier>: Entity<T> {}

#[cfg(test)]
mod tests {
    use super::{AggregateRoot, Entity, Identifier};
    use derive_more::Deref;

    #[test]
    pub fn test_define_user_id() {
        #[derive(Debug, Clone, PartialEq, Eq, Deref)]
        struct UserId(String);

        impl Identifier for UserId {}
    }

    #[test]
    pub fn test_define_user_entity() {
        #[derive(Debug, Clone, PartialEq, Eq, Deref)]
        struct UserId(String);

        impl Identifier for UserId {}

        struct User {
            id: UserId,
        }

        impl Entity<UserId> for User {
            fn id(&self) -> &UserId {
                &self.id
            }
        }
    }

    #[test]
    pub fn test_define_aggregate_root() {
        #[derive(Debug, Clone, PartialEq, Eq, Deref)]
        struct UserId(String);

        impl Identifier for UserId {}

        struct User {
            id: UserId,
        }

        impl Entity<UserId> for User {
            fn id(&self) -> &UserId {
                &self.id
            }
        }

        impl AggregateRoot<UserId> for User {}
    }
}
