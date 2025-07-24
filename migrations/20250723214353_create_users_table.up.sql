-- Add up a migration script here

create table users(
    id uuid primary key,
    name text not null,
    email text not null,
    is_enabled boolean default false,
    is_account_non_expired boolean default true,
    is_account_non_locked boolean default true,
    password varchar(255) not null,
    image_url text,
    constraint unique_email unique(email),
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create trigger set_updated_at
    before update on users
    for each row execute function update_updated_at_column();
