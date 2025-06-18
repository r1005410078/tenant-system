-- Add up migration script here
create table if not exists casbin_rules (
    id int auto_increment primary key,
    ptype varchar(255) not null,
    v0 varchar(255),
    v1 varchar(255),
    v2 varchar(255)
);