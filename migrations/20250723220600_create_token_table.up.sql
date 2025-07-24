-- Add up a migration script here

create table token(
    is_expired boolean not null,
    is_revoked boolean not null,
    created_at timestamp with time zone not null default now(),
    id  uuid not null primary key,
    user_id uuid not null constraint user_token_fk references users,
    token varchar(255)
);
