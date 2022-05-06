create table login_providers (
  user_id text not null,
  kind varchar(255) not null,
  id_in_provider varchar(255) not null
);

create table users (
  id integer not null,
  PRIMARY KEY (id)
);

