-- Add down migration script here
drop view if exists view_active_keycards;
create view view_active_keycards as
select sub.active_until, sub.door_id, sub.keycard_id, requester_id as user_id
from (select active_until, door_id, keycard_id, requester_id
      from tbl_request
               join tbl_door_to_request on tbl_request.request_id = tbl_door_to_request.request_id
      where tbl_request.active = true
        and (tbl_request.active_until > timezone('utc', now()) or tbl_request.active_until IS NULL)
        and tbl_request.accept = true
      union
      select active_until, null, keycard_id, requester_id
      from tbl_request
      where keycard_id IS NOT NULL
        and tbl_request.active = true
        and (tbl_request.active_until > timezone('utc', now()) or tbl_request.active_until IS NULL)
        and tbl_request.accept = true
      union
      select active_until, door_id, keycard_id, requester_id
      from tbl_request
               join tbl_request_department trd on tbl_request.request_id = trd.request_id
      join tbl_department td on trd.department_id = td.department_id
      join tbl_room_department t on td.department_id = t.department_id
      join tbl_door d on t.room_id = d.room_id
      where tbl_request.active = true
        and (tbl_request.active_until > timezone('utc', now()) or tbl_request.active_until IS NULL)
        and tbl_request.accept = true

      union
      select null, door_id, null, user_id as requester_id
      from tbl_door
      cross join tbl_user
      where tbl_user.role_id = 2

      ) as sub
    left join tbl_keycard on tbl_keycard.keycard_id = sub.keycard_id
    where tbl_keycard.given_out is not null and
          tbl_keycard.is_deactivated = false and
          tbl_keycard.is_locked = false and
          tbl_keycard.is_given_back = false
;

