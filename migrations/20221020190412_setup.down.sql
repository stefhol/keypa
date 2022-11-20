alter table tbl_keycard drop constraint if EXISTS fk_request_keycard;
alter table tbl_door_group drop constraint if exists fk_request_door_group;
drop table if exists tbl_door_access_history;

drop table if exists tbl_request_comment;
drop table if exists tbl_temp_keycard_request;
drop table if exists tbl_door_request;

drop table if exists tbl_door_to_group_door;
drop table if exists tbl_keycard_request;
drop table if exists tbl_request_base;
drop table if exists tbl_keycard_history;

drop table if exists tbl_keycard;
drop table if exists tbl_door_group;
drop table if exists tbl_door;

drop table if exists tbl_admin;

drop table if exists tbl_leader;

drop table if exists tbl_room;

drop table if exists tbl_building;
drop table if exists tbl_worker;
drop table if exists tbl_user;
drop table if exists tbl_role;
drop table if exists tbl_status;
drop table if exists _sqlx_migrations;