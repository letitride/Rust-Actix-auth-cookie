-- Your SQL goes here
create table directories (
  id bigserial not null,
  name varchar not null,
  parent bigint not null,
  created_at timestamp default current_timestamp not null,
  updated_at timestamp default current_timestamp not null,
  primary key (id)
)