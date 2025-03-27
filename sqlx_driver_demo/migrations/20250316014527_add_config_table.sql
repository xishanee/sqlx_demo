-- Add migration script here
create table kres.Config (
    id serial primary key,
    device_id integer not null,
    control_temperature numeric(5, 2),
    temperature_low_alarm_enable boolean,
    created_at timestamp with time zone not null default current_timestamp,
    foreign key (device_id) references kres.Device(id)
);

insert into kres.Config (device_id, control_temperature, temperature_low_alarm_enable) values (1, 100.00, true);