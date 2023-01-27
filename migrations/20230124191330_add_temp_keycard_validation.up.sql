-- Add up migration script here
drop view if exists view_active_keycards;
create view view_active_keycards as
with temp as (select sub.active_until, sub.door_id, sub.keycard_id, sub.requester_id as user_id
              from (select *
                    from (-- select every door that has an active request
                             select active_until, door_id, keycard_id, requester_id
                             from tbl_door_to_request
                                      join tbl_request tr on tbl_door_to_request.request_id = tr.request_id
                             where tr.active = true
                               and (tr.active_until > timezone('utc', now()) or tr.active_until IS NULL)
                               and tr.accept = true
                             union
-- select every department door that has an active request
                             select active_until, door_id, keycard_id, requester_id
                             from tbl_request_department trd
                                      join tbl_request tr on trd.request_id = tr.request_id
                                      join tbl_department td on trd.department_id = td.department_id
                                      join tbl_room_department t on td.department_id = t.department_id
                                      join tbl_door d on t.room_id = d.room_id
                             where tr.active = true
                               and (tr.active_until > timezone('utc', now()) or tr.active_until IS NULL)
                               and tr.accept = true) as doors

                    union
-- insert every door from the leader role_id 2
                    select null, door_id, null, user_id as requester_id
                    from tbl_door
                             cross join tbl_user
                    where tbl_user.role_id = 2) as sub
                       join tbl_user tu on tu.user_id = sub.requester_id
                       left join tbl_keycard tk on sub.keycard_id = tk.keycard_id
              where tu.is_active = true
                and tk.given_out is not null or tk is null
                and tk.is_deactivated = false or tk is null
                and tk.is_given_back = false or tk is null
                and tk.is_locked = false or tk is null
                and tk.is_lost = false or tk is null
              )
-- add keycards that are active and not in the previous result set
select * from (select active_until, null as door_id, tk.keycard_id, tk.user_id as user_id
from tbl_keycard tk
         join tbl_request tr on tk.keycard_id = tr.keycard_id
         join tbl_user tu on tk.user_id = tu.user_id
where tk.given_out is not null
  and tk.is_deactivated = false
  and tk.is_given_back = false
  and tk.is_locked = false
  and tk.is_lost = false
  and tr.active = true
  and (tr.active_until > timezone('utc', now()) or tr.active_until IS NULL)
  and tr.accept = true
  and tu.is_active = true
  and tk.keycard_id NOT IN (SELECT keycard_id FROM temp where keycard_id IS NOT NULL)
union
select *
from temp
) as sub;
