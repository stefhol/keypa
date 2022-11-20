create extension if not exists "uuid-ossp";
create table if not exists tbl_status(
    status_id uuid primary key DEFAULT uuid_generate_v4(),
    description text
);
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
    tel varchar(255),
    address varchar(255),
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
    foreign key(user_id) REFERENCES tbl_user(user_id)
);
create table if not exists tbl_leader(
    --leader is a user and a worker is a user
    user_id uuid primary key ,
    foreign key (user_id) references tbl_user(user_id)
);
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
    request_id uuid not null
);
create table if not exists tbl_keycard(
    keycard_id uuid primary key DEFAULT uuid_generate_v4(),
    request_id uuid not null
);
create table if not exists tbl_request_base(
    request_id uuid primary key DEFAULT uuid_generate_v4(),
    requester_id uuid not null,
    created_at timestamp without time zone not null default timezone('utc', now()),
    changed_at timestamp without time zone not null default timezone('utc', now()),
    active_until timestamp without time zone,
    description text,
    status_id uuid,
    accept boolean not null default false,
    reject boolean not null default false,
    pending boolean not null default true,
    foreign key (requester_id ) references tbl_user(user_id),
    foreign key(status_id) references tbl_status(status_id)
);

create table if not exists tbl_keycard_request(
    request_id uuid primary key,
    keycard_id uuid,
    is_lost bool not null default false,
    foreign key (keycard_id) references tbl_keycard(keycard_id),
    foreign key (request_id) references tbl_request_base(request_id)
);
alter table tbl_keycard ADD constraint fk_request_keycard foreign key (request_id) references tbl_request_base(request_id);
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
create table if not exists tbl_door_request(
    request_id uuid primary key,
    door_group_id uuid not null,
    foreign key (door_group_id) references tbl_door_group(door_group_id),
    foreign key (request_id) references tbl_request_base(request_id)
);
alter table tbl_door_group ADD CONSTRAINT fk_request_door_group foreign key (request_id) references tbl_request_base(request_id);
create table if not exists tbl_request_comment(
    comment_id uuid primary key DEFAULT uuid_generate_v4(),
    request_id uuid not null,
    user_id uuid not null,
    comment text not null,
    written_at timestamp without time zone not null default  timezone('utc', now()),
    foreign key (request_id) references tbl_request_base(request_id),
    foreign key (user_id) references tbl_user(user_id)
);

create table if not exists tbl_temp_keycard_request(
    request_id uuid primary key,
    keycard_id uuid,
    door_group_id uuid,
    is_payed bool not null DEFAULT false,
    is_given_back bool not null DEFAULT false,
    foreign key (keycard_id) references tbl_keycard(keycard_id),
    foreign key (door_group_id) references tbl_door_group(door_group_id),
    foreign key (request_id) references tbl_request_base(request_id)
);

-- History of Door access
create table if not exists tbl_door_access_history
(
    door_history_id uuid primary key default uuid_generate_v4(),
    door_id uuid not null,
    user_id uuid not null,
    foreign key(door_id) references tbl_door(door_id),
    foreign key (user_id) references tbl_user(user_id),
    deactivated_at timestamp without time zone,
    activated_at timestamp without time zone default timezone('utc', now())
);