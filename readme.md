# Rusite Server

为了弥补市面上一些博客框架上（例如 Hexo）不具备动态性的缺点，开发了这款基于 Rust Axum 实现的后端服务程序，利用 Rust 的优势实现了高并发，且保证了内存安全，具有更快的相应速度。主要实现的功能有以下几点：
  1. 留出了推送博文的接口，使得任何专业的人能够方便地进行存储并在网上发布博文。
  2. 留出了用户评论的接口，可以让用户对特定的文章发表评论，促进交流。
  3. 具有留言板/聊天室的功能，可以使得多人实时交流心得体会，留下足迹。
  4. 实现对已存储的博文进行分类，按照标签将博文进行分类，便于用户进行检索。
  5. 具有存储小文件的功能，留出了上传图片并存储的接口，可供用户制作相册等。
  6. 留出了修改网站信息的接口。

数据库采用了 MariaDB 来存储数据，设计了以下主要的表结构：
  1. users 用户表
  2. posts 博文表
  3. tags  标签表
  4. post_tags 博文标签链接表
  5. remarks 评论表
  6. chat_messages 留言板/聊天室消息表


## API Documentation

### Base URL

http://124.223.209.159:8300/api

### Routes

#### `/post`

##### Create Post

- **Description**: 创建一篇文章
- **URL**: `/post/create`
- **Method**: POST
- **Request Body**:
```json
{
   "title": "test",
   "author": 100000,
   "brief": "广告招租位",
   "content": "test",
   "tags": ["广告", "测试"]
}
```
- **Response**:
  - **Status**: 200 OK 

##### Delete Post

- **Description**: 删除一篇文章
- **URL**: `/post/delete`
- **Method**: POST
- **Request Body**:
```json
{
  "pid": "f354c332-a2ad-42a6-bdae-99b535395ef1"
}
```
- **Response**:
  - **Status**: 200 OK / 500 Internal Server Error
  - **Error Message**:
    ```json
    "Data Not Found in database"
    ```

##### Update Post

- **Description**: 更新一篇文章
- **URL**: `/post/update`
- **Method**: POST
- **Request Body**:
```json
{
  "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
  "title": "update",
  "brief": "广告招租位update",
  "content": "更新！！！！",
  "tags": ["广告", "测试", "更新"]
}
```
- **Response**:
  - **Status**: 200 OK

##### List Post Information

- **Description**: 得到文章列表
- **URL**: `/post/infolist`
- **Method**: GET
- **Response**:
  - **Status**: 200 OK
  - **Body**:
  ```json
    [
      {
        "author": 100000,
        "brief": "广告招租位",
        "date": "2024-06-06T13:08:27Z",
        "pid": "283036ea-afe6-46df-bbb4-f47d10172114",
        "tags": ["广告", "测试"],
        "title": "test"
      },
      ...
    ]
  ```

##### Get Post Content

- **Description**: 得到文章内容
- **URL**: `/post/content`
- **Method**: GET
- **Query Parameters**:
  - `pid`: string
- **Response**:
  - **Status**: 200 OK
  - **Body**:
  ```
    Content 内容 String
  ```

##### List Post Remarks

- **Description**: 得到文章对应的评论内容
- **URL**: `/post/remarks`
- **Method**: GET
- **Query Parameters**:
  - `pid`: string
- **Response**:
  - **Status**: 200 OK
  - **Body**:
  ```json
    [
      {
        "content": "remark",
        "created_at": "2024-06-06T13:08:27Z",
        "uid": 10000000
      },
      ...
    ]
  ```


#### `/user`

##### Register User

- **Description**: 注册一名用户
- **URL**: `/user/register`
- **Method**: POST
- **Request Body**:
```json
{
  "name": "username",
  "passwd": "password"
}
```
- **Response**:
  - **Status**: 200 OK
  - **Body**: 
   ```json
    uid
   ```

##### Delete User

- **Description**: 注销一名用户
- **URL**: `/user/delete`
- **Method**: POST
- **Request Body**:
```json
{
  "uid": 10000002
}
```
- **Response**:
  - **Status**: 200 OK
  
##### Update User

- **Description**: 更新用户信息
- **URL**: `/user/update`
- **Method**: POST
- **Request Body**:
```json
{
  "uid": 10000000,
  "passwd": "root",
  "name": "root",
  "avatar": null,
  "bio": null
}
```
- **Response**:
  - **Status**: 200 OK

##### User Login

- **Description**: 用户登陆
- **URL**: `/user/login`
- **Method**: POST
- **Request Body**:
```json
{
  "uid": 10000000,
  "passwd": "root"
}
```
- **Response**:
  - **Status**: 200 OK / 500 Internal Server Error
  - **Body**:
    - Success:
    ```json
    {
      "code": 1,
      "userinfo": {
        "avatar": null,
        "bio": null,
        "created_at": "2024-06-06T07:47:26Z",
        "name": "root",
        "uid": 10000000
      }
    }
    ```
    - Failure:
    ```json
    {
      "code": 0,
      "userinfo": null
    }
    ```
    - Error Message:
    ```
    "User Not Found
    ```

##### User Info

- **Description**: 得到用户信息
- **URL**: `/user/info`
- **Method**: GET
- **Query Parameters**:
  - `uid`: int
- **Response**:
  - **Status**: 200 OK
  - **Body**: 
  ```json
   {
    "uid": 10000000,
    "name": "root",
    "avatar": null,
    "bio": null,
    "created_at": "2024-06-06T07:47:26Z"
  }
  ```

##### Add Remark

- **Description**: 用户发表评论
- **URL**: `/user/remark`
- **Method**: POST
- **Request Body**:
```json
{
  "uid": 10000000,
  "pid": "dfebd659-9cd2-4c9a-8a42-85fbf561bd9a",
  "content": "remark"
}
```
- **Response**:
  - **Status**: 200 OK

##### Add Chat

- **Description**: 用户聊天
- **URL**: `/user/chat`
- **Method**: POST
- **Request Body**:
```json
{
  "uid": 10000000,
  "content": "chat"
}
```
- **Response**:
  - **Status**: 200 OK


#### `/chatlist`

- **Description**: 聊天消息列表
- **URL**: `/chatlist`
- **Method**: GET
- **Request Body**:
```json
{
  "uid": 10000000,
  "content": "chat"
}
```
- **Response**:
  - **Status**: 200 OK
  - **Body**:
  ```json
  [
    {
      "content": "chat",
      "created_at": "2024-06-06T12:55:05Z",
      "uid": 10000000
    },
    ...
  ]
  ```

#### Upload

- **Description**: 上传文件
- **URL**: `/upload`
- **Method**: POST
- **Request Body**:
  - Content-Type: multipart/form-data
  - Fields:
    - file: 文件内容，类型为 file，即要上传的文件。
- **Response**:
  - **Status**: 200 OK

> ps: 请传入的文件名做好唯一性区别，传入的文件内容要求小于 2MB

#### resouce

- **Description**: 静态文件访问
- **URL**: `/resouce/{file/path}`
- **Method**: GET
- **Response**:
  - **Status**: 200 OK


#### basic_info

- **Description**: 网站信息
- **URL**: `/basicinfo`
- **Method**: GET
- **Response**:
  - **Status**: 200 OK
  - **Body**:
  ```json
  {
    "sitebasicinfo": {
      "title": "",
      "subtitle": "",
      "description": "",
      "author": "",
      "favicon": "",
      "avatar": ""
    },
    "mylinks": {
      "github": "",
      "bilibili": "",
      "zhihu": "",
      "qq": "",
      "wechat": null,
      "gitee": ""
    }
  }
  ```

#### basic_info_update

- **Description**: 更新网站信息
- **URL**: `/updateinfo`
- **Method**: POST
- **Request Body**:
```json
{
  "sitebasicinfo": {
    "title": "",
    "subtitle": "",
    "description": "",
    "author": "",
    "favicon": "",
    "avatar": ""
  },
  "mylinks": {
    "github": "",
    "bilibili": "",
    "zhihu": "",
    "qq": "",
    "wechat": null,
    "gitee": ""
  }
}
```
- **Response**:
  - **Status**: 200 OK