use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{db::TagName, error::{Error, Result}, SafeMap, SafeVec, Uuid};

#[derive(Debug, Clone)]
pub struct UserInfo {

}

#[derive(Debug, Clone, Serialize)]
pub struct ClientPostInfo {
    pid: Uuid,
    title: String,
    date: DateTime<Utc>,
    author: u32,
    tags: Vec<String>,
    brief: String,
}

impl ClientPostInfo {
    pub fn new(
        pid: Uuid,
        title: String,
        date: DateTime<Utc>,
        author: u32,
        brief: String,
    ) -> Self {
        Self { pid, title, date, author, tags: Vec::new(), brief }
    }
    pub fn add_tag(&mut self, tag: TagName) {
        self.tags.push(tag.tag);
    }
}

impl From<PostInfo> for ClientPostInfo {
    fn from(p: PostInfo) -> Self {
        Self::new(p.pid, p.title, p.created_at, p.author, p.brief)
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct PostInfo {
    pub pid: Uuid,
    pub title: String,
    pub author: u32,
    pub brief: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub postinfo: PostInfo,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Clone)]
pub struct PostList {
    pub postlist: SafeVec<Post>,
}

impl PostList {
    pub async fn new() -> Self {
        Self {
            postlist: SafeVec::default(),
        }
    }
}

pub struct TagMap {
    tagmap: SafeMap<usize, String>,
}

impl TagMap {
    pub fn new() -> Self {
        Self {
            tagmap: SafeMap::default()
        }
    }
    pub fn insert_tag(&mut self, id: usize, name: String) -> Result<()> {
        let mut map = self.tagmap.lock().unwrap();
        match map.insert(id, name) {
            None => Ok(()),
            Some(_) => Err(Error::UnexpectedDataInModel),
        }
    }
    pub fn get(&self, id: usize) -> Option<String> {
        let map = self.tagmap.lock().unwrap();
        if let Some(c) = map.get(&id) {
            Some(String::from(c))
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub uid: u32,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub uid: u32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}