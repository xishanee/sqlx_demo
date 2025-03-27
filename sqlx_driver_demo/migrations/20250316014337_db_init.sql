-- Add migration script here
-- Add migration script here
CREATE SCHEMA kres;
ALTER SCHEMA kres OWNER TO postgres;

CREATE type kres.device_type AS ENUM ('ngc20', 'elexant5010i', 'elexant40x0i');

CREATE TABLE kres.Device (
    id SERIAL PRIMARY KEY,
    device_type kres.device_type NOT NULL,
    tag VARCHAR(255) NOT NULL,
    vendor VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

insert into kres.Device (device_type, tag, vendor) values ('ngc20', 'NGC20-TEST', 'Chemelex');
insert into kres.Device (device_type, tag, vendor) values ('elexant5010i', 'elexant 5010i test', 'nVent Thermal');
insert into kres.Device (device_type, tag) values ('elexant40x0i', 'elexant 4010i test');
