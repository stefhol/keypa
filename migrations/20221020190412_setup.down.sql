drop table if exists tbl_key_user_history;
drop table if exists tbl_request;

drop table if exists tbl_key_group_key;

drop table if exists tbl_key_group;

drop table if exists tbl_key;

drop table if exists tbl_door;

drop table if exists tbl_admin;

alter table tbl_worker DROP constraint if exists fk_worker_leader;

drop table if exists tbl_leader;

drop table if exists tbl_room;

drop table if exists tbl_building;
drop table if exists tbl_worker;
drop table if exists tbl_user;
drop table if exists tbl_role;
