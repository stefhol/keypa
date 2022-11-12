drop table if exists tbl_door_user_access;

drop table if exists tbl_request_comment;
drop table if exists tbl_request;

drop table if exists tbl_door_to_group_door;
drop table if exists tbl_keycard_history;
drop table if exists tbl_keycard;

drop table if exists tbl_door_group;

drop table if exists tbl_door;

drop table if exists tbl_admin;

alter table tbl_worker DROP constraint if exists fk_worker_leader;

drop table if exists tbl_leader;

drop table if exists tbl_room;

drop table if exists tbl_building;
drop table if exists tbl_worker;
drop table if exists tbl_user;
drop table if exists tbl_role;
drop table if exists _sqlx_migrations;