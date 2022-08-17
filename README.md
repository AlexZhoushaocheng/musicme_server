# Musicme Server
Musicme 的服务器，用来为客户端音乐资源。

## 计划的功能接口

* - [ ] 用户注册
* - [x] 登录
* - [x] 登出
* - [] 新建/删除音乐列表
* - [] 添加/删除音乐列表内容
* - [x] 查询音乐列表
* - [x] 获取音乐数据
* - [x] 搜索音乐（整个库中）
* - [x] 最爱（favorite）列表
* - [x] 分页浏览整个库


## 配置
### User table 
|id|name|create_time|
|---|---|---|
||||

### Song list info table
id|user_id|name|ar|create_time|
---|---|---|---|---|
||||



### Song list table
song_list_info_id|song_id
--|--