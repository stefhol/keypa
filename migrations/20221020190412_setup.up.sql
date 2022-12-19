create extension if not exists "uuid-ossp";
CREATE TYPE history_action AS ENUM ('remove', 'add', 'create','change');
CREATE TYPE history_role AS ENUM ('worker', 'user', 'leader', 'admin');
CREATE TYPE history_request_type AS ENUM ('keycard', 'door', 'temp');

create table if not exists tbl_status
(
    status_id   uuid primary key DEFAULT uuid_generate_v4(),
    description text
);

-- Extern User DB
create table if not exists tbl_role
(
    role_id     uuid primary key DEFAULT uuid_generate_v4(),
    name        varchar(255) not null,
    description text
);
create table if not exists tbl_user
(
    user_id     uuid primary key      DEFAULT uuid_generate_v4(),
    name        varchar(255) not null,
    role_id     uuid,
    foreign key (role_id) references tbl_role (role_id),
    is_active   bool         not null DEFAULT true,
    tel         varchar(255),
    address     varchar(255),
    email       varchar(255) not null,
    picture_url varchar(255),
    password    varchar(255) not null
);
-- End Extern User DB

-- Extern Room DB
create table if not exists tbl_building
(
    building_id uuid primary key DEFAULT uuid_generate_v4(),
    name        varchar(255) not null
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
    name    varchar(255) not null,
    room_id uuid         not null,
    foreign key (room_id) references tbl_room (room_id)
);
create table if not exists tbl_department
(
    department_id uuid primary key default uuid_generate_v4(),
    name          varchar(255) not null,
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

create table if not exists tbl_change_rights_history
(
    change_rights_history_id BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    action                   history_action              not null,
    internal_role            history_role                not null,
    target_user_id           uuid                        not null,
    changed_by               uuid                        not null,
    changed_at               timestamp without time zone not null default timezone('utc', now()),
    foreign key (target_user_id) references tbl_user (user_id),
    foreign key (changed_by) references tbl_user (user_id)
);


create table if not exists tbl_keycard
(
    keycard_id     uuid primary key DEFAULT uuid_generate_v4(),
    is_lost        bool    not null default false,
    is_locked      boolean not null default false,
    is_deactivated boolean not null default false,
    is_given_back  boolean not null default false,
    request_id     uuid    not null
);

create table if not exists tbl_request_entrance
(
    request_entrance_id uuid primary key default uuid_generate_v4(),
    request_id          uuid not null,

    building_id         uuid not null,
    foreign key (building_id) references tbl_building (building_id),
    proposed_rooms      text not null
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
    status_id    uuid,
    is_proposal  boolean                     not null default false,
    active       boolean                     not null default true,
    accept       boolean                     not null default false,
    reject       boolean                     not null default false,
    payed        boolean,
    pending      boolean                     not null default true,
    foreign key (requester_id) references tbl_user (user_id),
    foreign key (status_id) references tbl_status (status_id),
    keycard_id   uuid,
    foreign key (keycard_id) references tbl_keycard (keycard_id)
);
create table if not exists tbl_request_history
(
    request_history_id BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    changed_by         uuid                        not null,
    request_id         uuid                        not null,
    action             history_action              not null,
    request_type       history_request_type        not null,
    requester_id       uuid                        not null,
    created_at         timestamp without time zone not null,
    changed_at         timestamp without time zone not null,
    active_until       timestamp without time zone,
    description        text,
    status_id          uuid,
    active             boolean                     not null,
    accept             boolean                     not null,
    reject             boolean                     not null,
    pending            boolean                     not null,
    keycard_id         uuid,
    is_lost            bool                        not null default false,
    is_payed           bool                        not null DEFAULT false,
    is_given_back      bool                        not null DEFAULT false,
    foreign key (keycard_id) references tbl_keycard (keycard_id),
    foreign key (requester_id) references tbl_user (user_id),
    foreign key (changed_by) references tbl_user (user_id),
    foreign key (request_id) references tbl_request (request_id),
    foreign key (status_id) references tbl_status (status_id)
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
alter table tbl_request_entrance
    ADD constraint fk_request_request_entrance foreign key (request_id) references tbl_request (request_id);
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
create table if not exists tbl_log
(
    log_id                     BIGINT GENERATED ALWAYS AS IDENTITY primary key,
    message                    text,
    keycard_history_id         bigint,
    foreign key (keycard_history_id) references tbl_keycard_history (keycard_history_id),
    door_to_request_history_id bigint,
    foreign key (door_to_request_history_id) references tbl_door_to_request_history (door_to_request_history_id),
    request_history_id         bigint,
    foreign key (request_history_id) references tbl_request_history (request_history_id),
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