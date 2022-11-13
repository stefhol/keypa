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
    is_active bool not null DEFAULT true,
    email varchar(255) not null,
    picture_url varchar(255),
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
    user_id uuid primary key,
    foreign key(user_id) REFERENCES tbl_user(user_id),
    --worker has a leader foreign key gets added later
    boss_user_id uuid
);
create table if not exists tbl_leader(
    --leader is a user and a worker is a user
    user_id uuid primary key ,
    foreign key (user_id) references tbl_user(user_id)
);
alter table tbl_worker add constraint fk_worker_leader foreign key(boss_user_id) references tbl_leader(user_id);
create table if not exists tbl_admin(
    user_id uuid primary key,
    foreign key (user_id) references tbl_user(user_id)
);
create table if not exists tbl_door(
    door_id uuid primary key DEFAULT uuid_generate_v4(),
    name varchar(255) not null,
    room_id uuid not null,
    foreign key (room_id) references tbl_room(room_id)
);

--group of doors
create table if not exists tbl_door_group(
    door_group_id uuid primary key DEFAULT uuid_generate_v4(),
    owner_id uuid not null,
    foreign key (owner_id) references tbl_user(user_id),
    name varchar(255) not null,
    description text
);
create table if not exists tbl_keycard(
    keycard_id uuid primary key DEFAULT uuid_generate_v4(),
    user_id uuid not null,
    active bool not null DEFAULT false,
    active_until timestamp without time zone,
    foreign key (user_id) references tbl_user(user_id)
);
create table if not exists tbl_keycard_history(
    keycard_id uuid not null,
    door_id uuid not null,
    used_at timestamp without time zone not null default timezone('utc', now()),
    primary key (door_id,keycard_id,used_at),
    foreign key (keycard_id) references tbl_keycard(keycard_id),
    foreign key (door_id) references tbl_door(door_id)
);
--group of keys n:n key
create table if not exists tbl_door_to_group_door(
    door_id uuid,
    door_group_id uuid,
    foreign key (door_id) references tbl_door(door_id),
    foreign key (door_group_id) references tbl_door_group(door_group_id),
    primary key (door_id,door_group_id)
);
--request to get a key
create table if not exists tbl_request(
    request_id uuid primary key DEFAULT uuid_generate_v4(),
    requester_id uuid not null,
    foreign key (requester_id) references tbl_user(user_id),
    door_group_id uuid not null,
    foreign key (door_group_id) references tbl_door_group(door_group_id),
    created_at timestamp without time zone not null default timezone('utc', now()),
    changed_at timestamp without time zone not null default timezone('utc', now()),
    description text,
    accept boolean not null default false,
    reject boolean not null default false,
    pending boolean not null default true
);
create table if not exists tbl_request_comment(
    comment_id uuid primary key DEFAULT uuid_generate_v4(),
    request_id uuid not null,
    user_id uuid not null,
    comment text not null,
    written_at timestamp without time zone not null default  timezone('utc', now()),
    foreign key (request_id) references tbl_request(request_id),
    foreign key (user_id) references tbl_user(user_id)
);
-- History of Door access
-- Add all doors from door_group to history
create table if not exists tbl_door_user_access(
    door_id uuid not null,
    user_id uuid not null,
    primary key (door_id,user_id),
    foreign key(door_id) references tbl_door(door_id),
    foreign key (user_id) references tbl_user(user_id),
    due_at timestamp without time zone,
    lent_at timestamp without time zone default timezone('utc', now()),
    lent boolean default false,
    is_active boolean default false,
    -- flag if a problem occurred
    has_problem boolean default false,
    -- comment
    comment text
);