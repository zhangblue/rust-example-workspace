create
database axum_example_db;

-- 票据表
create table tickets
(
    id      char(36) PRIMARY KEY not null, -- 票据id
    user_id int                  not null, -- 票据所属用户的id
    title   varchar(50)          not null  -- 票据头
);

-- 用户表
create table users
(
    id          char(36) PRIMARY KEY not null, -- 用户id
    account     varchar(50)          not null, -- 用户账号
    password    varchar(50)          not null, -- 用户密码
    nick_name   varchar(50)          not null, -- 昵称
    create_time timestamp            not null  -- 用户创建时间
);


-- 使用 sea-orm-cli 生成entity内容 ： sea-orm-cli generate entity -u postgres://postgres:12345678@localhost/axum_example_db -o axum-example/src/entity