create table accounts (
    id varchar(64) not null primary key,
    email varchar(320) not null unique,
    password_hash varchar(255) not null,
    display_name varchar(255),
    created_at timestamp with time zone
);
