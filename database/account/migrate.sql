create table login_providers (
  user_id text not null,
  kind varchar(255) not null,
  id_in_provider varchar(255) not null,
  updated_at timestamp without time zone not null,
  PRIMARY KEY (kind, id_in_provider)
);

create table users (
  id varchar(255) not null,
  updated_at timestamp without time zone not null,
  PRIMARY KEY (id)
);

create table profiles (
  user_id varchar(255) not null,
  name varchar(20) not null,
  display_name text not null,
  avatar_url text not null,
  updated_at timestamp without time zone not null,
  primary key (user_id)
);

