drop view view_active_doors_by_user;
drop view view_active_keycards;

alter table tbl_keycard drop constraint if EXISTS fk_request_keycard;
drop table if exists tbl_door_access_history;
drop table if exists tbl_change_rights_history;
drop table if exists tbl_request_history;
drop table if exists tbl_door_to_request_history;
drop table if exists tbl_request_comment;
drop table if exists tbl_temp_keycard_request;
drop table if exists tbl_door_request;

-- proposal
drop table if exists tbl_proposal_entrance;
drop table if exists tbl_proposal_department;
drop table if exists tbl_proposal;

--request
drop table if exists tbl_door_to_request;
drop table if exists tbl_keycard_request;
drop table if exists tbl_request_base;
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
DROP TYPE history_action;
DROP TYPE history_role;
DROP TYPE history_request_type;
drop table if exists _sqlx_migrations;