use std::{env, result, time::Duration};

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use crate::{error::{Error, Result}, model::{Message, PostInfo, User}, Uuid};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<MySql>,
}

impl Database {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        Self {
            pool: MySqlPoolOptions::new()
                .max_connections(20)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&url)
                .await
                .expect("Can't connect Database")
        }
    }
}

impl Database {

    pub async fn get_post_list(
        &self
    ) -> Result<Vec<PostInfo>> {
        let postinfo_list: Vec<PostInfo> = sqlx::query_as::<_, PostInfo> (
            r#"
SELECT pid, title, author, brief, created_at, updated_at from posts
ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(postinfo_list)
    }

    pub async fn push_post(
        &self,
        pid: &Uuid,
        title: &str,
        author: u32,
        brief: &str,
        content: &str,
        tags: &Vec<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
INSERT INTO posts (pid, title, author, brief, content)
VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(pid)
        .bind(title)
        .bind(author)
        .bind(brief)
        .bind(content)
        .execute(&self.pool)
        .await?;
        self.push_tags(pid, tags).await?;
        Ok(())
    }

    pub async fn push_tags(
        &self,
        pid: &Uuid,
        tags: &Vec<String>,
    ) -> Result<()> {
        for tag in tags {
            sqlx::query(
                r#"
INSERT INTO post_tag (pid, tag)
VALUES (?, ?)
                "#
            )
            .bind(pid)
            .bind(tag)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn delete_post(
        &self,
        pid: &Uuid,
    ) -> Result<()> {
        sqlx::query("DELETE FROM post_tag WHERE pid = ?").bind(pid).execute(&self.pool).await?;
        let res =  sqlx::query(
            r#"
DELETE FROM posts WHERE pid = ?
            "#
        )
        .bind(pid)
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn update_post(
        &self,
        pid: &Uuid,
        title: &str,
        brief: &str,
        content: &str,
        tags: &Vec<String>,
    ) -> Result<()> {
        sqlx::query("DELETE FROM post_tag WHERE pid = ?").bind(pid).execute(&self.pool).await?;
        sqlx::query(
            r#"
UPDATE posts
SET title = ?, brief = ?, content = ?, updated_at = current_timestamp()
WHERE pid = ?
            "#
        )
        .bind(title)
        .bind(brief)
        .bind(content)
        .bind(pid)
        .execute(&self.pool)
        .await?;
        self.push_tags(pid, tags).await?;
        Ok(())
    }

    pub async fn get_post_alltag(
        &self,
        pid: &Uuid,
    ) -> Result<Vec<TagName>> {
        let tags: Vec<TagName> = sqlx::query_as::<_, TagName> (
            r#"
SELECT tag FROM post_tag WHERE pid = ?
            "#
        )
        .bind(pid)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(tags)
    }

    pub async fn get_post_conent(
        &self,
        pid: &Uuid,
    ) -> Result<String> {
        let content = sqlx::query_scalar(
            r#"
SELECT content FROM posts WHERE pid = ?
            "#
        )
        .bind(pid)
        .fetch_one(&self.pool)
        .await?;
        Ok(content)
    }

    pub async fn get_user_name(
        &self,
        uid: u32,
    ) -> Result<String> {
        let name: String = sqlx::query_scalar(
            r#"
SELECT name FROM user WHERE uid = ?
            "#
        )
        .bind(uid)
        .fetch_one(&self.pool)
        .await?;

        Ok(name)
    }

    pub async fn register_user(
        &self,
        name: &str,
        passwd: &str,
    ) -> Result<u32> {
        let uid = sqlx::query(
            r#"
INSERT INTO users (name, passwd)
VALUES (?, ?)
            "#,
        )
        .bind(name)
        .bind(passwd)
        .execute(&self.pool)
        .await?
        .last_insert_id() as u32;
        Ok(uid)
    }

    pub async fn delete_user(
        &self,
        uid: u32,
    ) -> Result<()> {
        sqlx::query(
            r#"
DELETE FROM users WHERE uid = ?
            "#
        )
        .bind(uid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user(
        &self,
        uid: u32,
        passwd: &str,
        name: &str,
        avatar: &Option<String>,
        bio: &Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
UPDATE users
SET passwd = ?, name = ?, avatar = ?, bio = ?, updated_at = current_timestamp()
WHERE uid = ?
            "#
        )
        .bind(passwd)
        .bind(name)
        .bind(avatar)
        .bind(bio)
        .bind(uid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn check_passwd(
        &self,
        uid: u32,
        passwd: &str,
    ) -> Result<bool> {
        let sqlres: result::Result<String, sqlx::Error> = sqlx::query_scalar(
            r#"
SELECT passwd FROM users WHERE uid = ?
            "#
        )
        .bind(uid)
        .fetch_one(&self.pool)
        .await;
        if let Ok(checkpasswd) = sqlres {
            if checkpasswd.eq(passwd){
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub async fn get_user_info(
        &self,
        uid: u32,
    ) -> Result<User> {
        let userinfo: User = sqlx::query_as::<_, User> (
            r#"
SELECT uid, name, avatar, bio, created_at FROM users WHERE uid = ?
            "#
        )
        .bind(uid)
        .fetch_one(&self.pool)
        .await?;
        Ok(userinfo)
    }

    pub async fn push_remark(
        &self,
        uid: u32,
        pid: &Uuid,
        content: &str
    ) -> Result<()> {
        sqlx::query(
            r#"
INSERT INTO remarks (uid, pid, content)
VALUES (?, ?, ?)
            "#
        )
        .bind(uid)
        .bind(pid)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_remark(
        &self,
        pid: &Uuid,
    ) -> Result<Vec<Message>> {
        let remarks: Vec<Message> = sqlx::query_as::<_, Message>(
            r#"
SELECT uid, content, created_at FROM remarks WHERE pid = ? ORDER BY created_at DESC
            "#
        )
        .bind(pid)
        .fetch_all(&self.pool)
        .await?;
        Ok(remarks)
    }

    pub async fn list_chat(
        &self
    ) -> Result<Vec<Message>> {
        let chatmsg: Vec<Message> = sqlx::query_as::<_, Message> (
            r#"
SELECT uid, content, created_at FROM chat_messages ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(chatmsg)
    }

    pub async fn push_chatmsg(
        &self,
        uid: u32,
        content: &str
    ) -> Result<()> {
        sqlx::query(
            r#"
INSERT INTO chat_messages (uid, content)
VALUES (?, ?)
            "#
        )
        .bind(uid)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

}

#[derive(sqlx::FromRow)]
pub struct TagName {
    pub tag: String,
}