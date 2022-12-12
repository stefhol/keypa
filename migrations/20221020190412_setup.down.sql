drop view if exists view_active_doors_by_user;
-- drop view view_active_keycards;

alter table if exists tbl_keycard drop constraint if EXISTS fk_request_keycard;
alter table if exists tbl_request_entrance drop constraint if EXISTS fk_request_request_entrance;
alter table if exists tbl_request_department drop constraint if EXISTS fk_request_request_department;

drop table if exists tbl_change_rights_history;
drop table if exists tbl_request_history;
drop table if exists tbl_door_to_request_history;
drop table if exists tbl_door_to_request;
drop table if exists tbl_request_comment;
drop table if exists tbl_temp_keycard_request;
drop table if exists tbl_door_request;
drop table if exists tbl_request_department;
drop table if exists tbl_request_entrance;

-- proposal
drop table if exists tbl_proposal_entrance;
drop table if exists tbl_proposal_department;
drop table if exists tbl_proposal;

--request
drop table if exists tbl_request;
drop table if exists tbl_keycard_history;

drop table if exists tbl_keycard;

-- user stuff
drop table if exists tbl_admin;
drop table if exists tbl_worker;
drop table if exists tbl_leader;
-- extern room stuff
drop table if exists tbl_room_department;
drop table if exists tbl_department;
drop table if exists tbl_door;
drop table if exists tbl_room;
drop table if exists tbl_building;

-- extern user stuff
drop table if exists tbl_user;
drop table if exists tbl_role;
drop table if exists tbl_status;
DROP TYPE if exists history_action;
DROP TYPE if exists history_role;
DROP TYPE if exists history_request_type;
drop table if exists _sqlx_migrations;