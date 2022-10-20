create extension if not exists "uuid-ossp";
create table if not exists tbl_role(
    role_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    description text
);
create table if not exists tbl_user(
    user_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    role_id uuid,
    foreign key (role_id ) references tbl_role(role_id),
    email varchar(255) not null,
    password varchar(255) not null
);

create table if not exists tbl_building(
  building_id uuid primary key DEFAULT uuid_generate_v4(),
  name varchar(255) not null
);
create table if not exists tbl_room(
    room_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    floor integer not null,
    is_sensitive boolean DEFAULT false,
    building_id uuid not null,
    foreign key(building_id) references tbl_building(building_id)
);
create table if not exists tbl_worker(
    worker_id uuid primary key DEFAULT uuid_generate_v4(),
    user_id uuid not null,
    foreign key(user_id) REFERENCES tbl_user(user_id),
    --worker has a leader foreign key gets added later
    leader_id uuid
);
create table if not exists tbl_leader(
    leader_id uuid primary key DEFAULT uuid_generate_v4(),
    --leader is a worker and a worker has a user
    worker_id uuid not null,
    foreign key (worker_id) references tbl_worker(worker_id)
);
alter table tbl_worker add constraint fk_worker_leader foreign key(leader_id) references tbl_leader(leader_id);
create table if not exists tbl_admin(
    admin_id uuid primary key DEFAULT uuid_generate_v4(),
    user_id uuid not null,
    foreign key (user_id) references tbl_user(user_id)
);
create table if not exists tbl_door(
    door_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    room_id uuid,
    foreign key (room_id) references tbl_room(room_id)
);
create table if not exists tbl_key(
    key_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    value varchar(255) not null,
    description text,
    door_id uuid not null,
    foreign key (door_id) references tbl_door(door_id)
);
--group of keys
create table if not exists tbl_key_group(
    key_group_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    description text
);
--group of keys n:n key
create table if not exists tbl_key_group_key(
    key_id uuid,
    key_group_id uuid,
    foreign key (key_id) references tbl_key(key_id),
    foreign key (key_group_id) references tbl_key_group(key_group_id),
    primary key (key_id,key_group_id)
);
--request to get a key
create table if not exists tbl_request(
    request_id uuid primary key DEFAULT uuid_generate_v4(),
    requester_id uuid not null,
    foreign key (requester_id) references tbl_user(user_id),
    key_group_id uuid not null,
    foreign key (key_group_id) references tbl_key_group(key_group_id),
    description text,
    accept boolean,
    reject boolean,
    pending boolean
);
-- History of Keys
-- Add all keys from key_group to history
create table if not exists tbl_key_user_history(
    key_id uuid not null,
    user_id uuid not null,
    primary key (key_id,user_id),
    foreign key(key_id) references tbl_key(key_id),
    foreign key (user_id) references tbl_user(user_id),
    lent_at timestamp,
    due_at timestamp,
    lent boolean default false,
    -- the time key starts to become active for example 8am
    active_at time,
    -- duration the key is active in seconds
    active_duration integer,
    -- override if active is enabled
    is_active boolean default false,
    -- flag if a problem occurred
    has_problem boolean default false,
    -- comment
    comment text
);