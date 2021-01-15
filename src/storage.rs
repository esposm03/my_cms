use crate::routes::{Post, Tag};

use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;
use owning_ref::{RwLockReadGuardRef, RwLockWriteGuardRefMut};

pub struct Storage {
    tags: RwLock<HashMap<String, Tag>>,
    posts: RwLock<HashMap<Uuid, Post>>,
}
impl juniper::Context for Storage {}

impl Storage {
    pub fn new(tags: HashMap<String, Tag>, posts: HashMap<Uuid, Post>) -> Self {
        Self {
            tags: RwLock::new(tags),
            posts: RwLock::new(posts),
        }
    }
    pub fn posts(&self) -> RwLockReadGuardRef<HashMap<Uuid, Post>> {
        RwLockReadGuardRef::new(self.posts.read().unwrap())
    }
    pub fn posts_mut(&self) -> RwLockWriteGuardRefMut<HashMap<Uuid, Post>> {
        RwLockWriteGuardRefMut::new(self.posts.write().unwrap())
    }
}