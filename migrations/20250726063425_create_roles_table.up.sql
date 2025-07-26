-- Add up migration script here

create table roles(
    id uuid primary key,
    user_id uuid not null constraint user_role_fk references users,
    role text not null
)