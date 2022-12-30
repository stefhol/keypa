create extension if not exists "uuid-ossp";
CREATE TYPE history_action AS ENUM ('remove', 'add', 'create','change');
CREATE TYPE history_request_type AS ENUM ('keycard', 'door', 'temp');

-- Extern User DB
create table if not exists tbl_user
(
    user_id     uuid primary key      DEFAULT uuid_generate_v4(),
    name        varchar(255) not null,
    role_id     bigint,
    is_active   bool         not null DEFAULT true,
    tel         varchar(255),
    address     varchar(255),
    email       varchar(255) unique not null,
    picture_url varchar(255),
    password    varchar(255) not null
);
-- End Extern User DB

-- Extern Room DB
create table if not exists tbl_building
(
    building_id uuid primary key DEFAULT uuid_generate_v4(),
    name        varchar(255) unique not null
);

create table if not exists tbl_room
(
    room_id      uuid primary key DEFAULT uuid_generate_v4(),
    name         varchar(255) not null,
    floor        integer      not null,
    is_sensitive boolean          DEFAULT false,
    building_id  uuid         not null,
    foreign key (building_id) references tbl_building (building_id)
);
create table if not exists tbl_door
(
    door_id uuid primary key DEFAULT uuid_generate_v4(),
    room_id uuid         not null,
    foreign key (room_id) references tbl_room (room_id)
);
create table if not exists tbl_department
(
    department_id uuid primary key default uuid_generate_v4(),
    name          varchar(255) unique not null,
    description   text
);
create table if not exists tbl_room_department
(
    department_id uuid not null,
    room_id       uuid not null,
    primary key (department_id, room_id),
    foreign key (department_id) references tbl_department (department_id),
    foreign key (room_id) references tbl_room (room_id)

);
-- End Extern Room DB

create table if not exists tbl_keycard
(
    keycard_id     uuid primary key DEFAULT uuid_generate_v4(),
    user_id uuid not null,
    foreign key(user_id) references tbl_user(user_id),
    is_lost        boolean    not null default false,
    is_locked      boolean not null default false,
    is_deactivated boolean not null default false,
    is_given_back  boolean not null default false,
    request_id     uuid,
    given_out  timestamp without time zone
);create table if not exists tbl_keycard_archive
(
    keycard_id     uuid primary key DEFAULT uuid_generate_v4(),
    user_id uuid not null,
    foreign key(user_id) references tbl_user(user_id),
    is_lost        boolean    not null default false,
    is_locked      boolean not null default false,
    is_deactivated boolean not null default false,
    is_given_back  boolean not null default false,
    given_out  timestamp without time zone
);


create table if not exists tbl_request_department
(
    request_id    uuid not null,
    department_id uuid not null,
    primary key (request_id, department_id),

    foreign key (department_id) references tbl_department (department_id)

);

create table if not exists tbl_request
(
    request_id   uuid primary key                     DEFAULT uuid_generate_v4(),
    requester_id uuid                        not null,
    created_at   timestamp without time zone not null default timezone('utc', now()),
    changed_at   timestamp without time zone not null default timezone('utc', now()),
    active_until timestamp without time zone,
    description  text,
    additional_rooms  text,
    active       boolean                     not null default true,
    accept       boolean                     not null default false,
    reject       boolean                     not null default false,
    payed        boolean,
    pending      boolean                     not null default true,
    foreign key (requester_id) references tbl_user (user_id),
    keycard_id   uuid,
    foreign key (keycard_id) references tbl_keycard (keycard_id)
);
create table if not exists tbl_door_to_request_history
(
    door_to_request_history_id BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    door_id                    uuid           not null,
    request_id                 uuid           not null,
    action                     history_action not null,
    changed_by                 uuid           not null,
    foreign key (door_id) references tbl_door (door_id),
    foreign key (request_id) references tbl_request (request_id),
    foreign key (changed_by) references tbl_user (user_id)
);
alter table tbl_keycard
    ADD constraint fk_request_keycard foreign key (request_id) references tbl_request (request_id);
alter table tbl_request_department
    ADD constraint fk_request_request_department foreign key (request_id) references tbl_request (request_id);
create table if not exists tbl_keycard_history
(
    keycard_history_id BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    keycard_id         uuid                        not null,
    door_id            uuid                        not null,
    used_at            timestamp without time zone not null default timezone('utc', now()),
    success            boolean                     not null,
    foreign key (keycard_id) references tbl_keycard (keycard_id),
    foreign key (door_id) references tbl_door (door_id)
);

create table if not exists tbl_door_to_request
(
    door_id    uuid,
    request_id uuid,
    foreign key (door_id) references tbl_door (door_id),
    foreign key (request_id) references tbl_request (request_id),
    primary key (door_id, request_id)
);

create table if not exists tbl_request_comment
(
    comment_id uuid primary key                     DEFAULT uuid_generate_v4(),
    request_id uuid                        not null,
    user_id    uuid                        not null,
    comment    text                        not null,
    written_at timestamp without time zone not null default timezone('utc', now()),
    foreign key (request_id) references tbl_request (request_id),
    foreign key (user_id) references tbl_user (user_id)
);
create table if not exists tbl_request_log
(
    log_id                     BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    message                    text,
    keycard_history_id         bigint,
    foreign key (keycard_history_id) references tbl_keycard_history (keycard_history_id),
    door_to_request_history_id bigint,
    foreign key (door_to_request_history_id) references tbl_door_to_request_history (door_to_request_history_id),
    changed_at                 timestamp without time zone not null default timezone('utc', now()),
    changed_by                 uuid                        not null,
    foreign key (changed_by) references tbl_user (user_id)
);

create view view_active_doors_by_user as
select tbl_request.requester_id as user_id, tbl_door_to_request.door_id, tbl_request.active_until
from tbl_request
         join tbl_door_to_request on tbl_request.request_id = tbl_door_to_request.request_id
where tbl_request.active = true;
-- Nutzer Id, Tür Id und Ablaufzeit des Zugriffs von Nutzer auf die Tür