-- Add up migration script here

alter table users add column source varchar(30) not null default 'SYSTEM'