
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- CREATE TYPE material_type AS ENUM ('PLA', 'PETG', 'ABS', 'Composite', 'Resin');

create table if not exists file
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    fullname          varchar(255) not null,
    created       timestamp    WITH TIME ZONE DEFAULT NOW(),
    sizebytes        bigint       not null,
    downloads     integer      default 0,
    average_rating real         default 0
    );


create table if not exists gcode
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    readme text,
    file_pk UUID not null
    constraint gcode_file_fk
    references file
    );


create table if not exists material_brand
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    full_name varchar(100) not null
    );


create table if not exists material
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    description             varchar(100)       not null,
    mat_type          varchar(24) not null,
    material_brand_fk uuid       not null
    constraint material_material_brand_id_fk
    references material_brand
    );

create table if not exists printer_brand
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    full_name varchar(100) not null
    );


create table if not exists printer
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    model            varchar(100) not null,
    printer_brand_fk uuid      not null
    constraint printer_printer_brand_id_fk
    references printer_brand
    );

create table if not exists print
(
    id uuid DEFAULT (uuid_generate_v4()) PRIMARY KEY NOT NULL,
    material_fk      uuid
    constraint print_material_id_fk
    references material,
    printer_fk       uuid
    constraint print_printer_id_fk
    references printer,
    gcode_fk         uuid not null
    constraint print_gcode_id_fk
    references gcode,
    nozzle_size_mm   double precision,
    bed_temp_celsius integer,
    successful       boolean default true,
    extruder_temp    integer
    );


create table if not exists user_account
(
    mail varchar(100) NOT NULL PRIMARY KEY,
    user_name varchar(100) NOT NULL,
    UNIQUE (mail, user_name)
    );


create table if not exists file_permissions
(
    permission varchar(10) PRIMARY KEY NOT NULL
    );

create table if not exists files_per_user
(
    user_account_pk varchar(100) not null
    constraint files_per_user_user_account_fk
    references user_account,
    roles_pk varchar(10) not null
    constraint files_per_user_roles_fk
    references file_permissions,
    files_pk uuid not null
    constraint files_per_user_file_fk
    references file,
    unique (user_account_pk, files_pk)
    );

INSERT INTO user_account (mail, user_name) VALUES ('mail1@mail.de', 'User 1');
INSERT INTO user_account (mail, user_name) VALUES ('mail2@mail.de', 'User 2');
INSERT INTO user_account (mail, user_name) VALUES ('mail3@mail.de', 'User 3');
INSERT INTO user_account (mail, user_name) VALUES ('mail4@mail.de', 'User 4');
INSERT INTO user_account (mail, user_name) VALUES ('mail5@mail.de', 'User 5');


INSERT INTO file_permissions (permission) VALUES ('owner');
INSERT INTO file_permissions (permission) VALUES ('delete');
INSERT INTO file_permissions (permission) VALUES ('read');


INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('jazzcat.stl', '2022-05-01 10:30:00.000000', 83778582, 10, 4);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('quantumquark.stl', '2022-02-14 13:45:00.000000', 60135415, 5, 2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('silverstar.stl', '2022-08-27 08:20:00.000000', 94601664, 8, 3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('neonbutterfly.stl', '2022-11-21 16:10:00.000000', 22205111, 3, 3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('astrodrone.stl', '2022-04-08 09:00:00.000000', 34657510, 12, 3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('cyberpuma.stl', '2022-09-05 14:15:00.000000', 3309023, 7, 2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('frostblade.stl', '2022-06-30 11:25:00.000000', 48661671, 9, 2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('emeraldgolem.stl', '2022-03-12 15:50:00.000000', 71546675, 6, 2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('glitchunicorn.stl', '2022-10-17 07:40:00.000000', 28312756, 4, 4);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('dystopianangel.stl', '2022-07-23 12:35:00.000000', 47018976, 11, 2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Schachfigur_König.stl', '2022-03-01 12:00:00.000000', 120352, 542, 4.8);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Roboterarm.obj', '2022-04-15 10:30:00.000000', 874201, 127, 4.2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Ring.stl', '2022-03-27 10:12:34.000000', 203456, 12, 4.5);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Vase.stl', '2022-02-19 09:32:45.000000', 405632, 8, 3.2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Lampshade.stl', '2022-03-31 14:22:10.000000', 137890, 15, 4);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Bottle.stl', '2022-02-28 17:30:25.000000', 267890, 20, 4.8);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Chair.stl', '2022-01-15 08:45:11.000000', 356789, 6, 3.7);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Table.stl',  '2022-03-10 11:20:22.000000', 789012, 10, 4.2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Lamp.stl', '2022-01-05 13:55:45.000000', 450230, 18, 4.7);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Sculpture.stl', '2022-02-09 16:40:19.000000', 612340, 9, 3.5);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Cup.stl', '2022-03-15 07:17:57.000000', 235678, 14, 4.1);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('ChessPiece.stl', '2022-02-14 12:30:40.000000', 190567, 7, 3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('FlowerPot.stl', '2022-01-21 18:02:53.000000', 354890, 11, 4.3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('KeyChain.stl', '2022-03-07 09:25:36.000000', 120456, 23, 4.9);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('ToyCar.stl', '2022-02-01 14:50:00.000000', 290345, 5, 3.8);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Bookend.stl', '2022-03-28 16:15:05.000000', 176543, 16, 4.4);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('HeadphoneStand.stl', '2022-01-17 11:45:30.000000', 456789, 13, 3.9);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Planter.stl', '2022-03-03 20:30:42.000000', 298765, 17, 4.6);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('WallHook.stl', '2022-02-12 15:00:12.000000', 234567, 10, 3.3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Bowl.stl', '2022-01-09 13:20:15.000000', 398765, 8, 3.1);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('CookieCutter.stl', '2022-03-23 09:10:20.000000', 142345, 11, 1.2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Robot Arm.stl','2022-01-05 10:15:00.000000', 10245328, 213, 4.5);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Table Lamp.stl', '2021-12-20 09:30:00.000000', 3237592, 97, 4.2);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Door Handle.stl', '2022-02-12 14:20:00.000000', 248930, 45, 4);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Shower Head.stl', '2021-11-08 16:45:00.000000', 1693248, 128, 4.6);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Bookshelf.stl', '2021-09-22 11:10:00.000000', 10485760, 342, 4.1);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Guitar.stl', '2022-03-18 13:25:00.000000', 8110248, 157, 4.3);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Toaster.stl', '2021-10-14 15:40:00.000000', 346827, 68, 4.1);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Keychain.stl', '2022-04-01 08:55:00.000000', 12964, 23, 4.8);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Eiffel Tower.stl', '2021-12-03 12:45:00.000000', 6291456, 241, 4.7);
INSERT INTO file (fullname, created, sizebytes, downloads, average_rating) VALUES ('Flower Pot.stl', '2022-02-22 17:30:00.000000', 1425856, 78, 4.2);

INSERT INTO printer_brand (full_name) VALUES ('Creality');
INSERT INTO printer_brand (full_name) VALUES ('Anycubic');
INSERT INTO printer_brand (full_name) VALUES ('Prusa');
INSERT INTO printer_brand (full_name) VALUES ('Ultimaker');
INSERT INTO printer_brand (full_name) VALUES ('LulzBot');


INSERT INTO material_brand (full_name) VALUES ('Prusa Research');
INSERT INTO material_brand (full_name) VALUES ('MakerBot');
INSERT INTO material_brand (full_name) VALUES ('Ultimaker');
INSERT INTO material_brand (full_name) VALUES ('Hatchbox');
INSERT INTO material_brand (full_name) VALUES ('eSun');
INSERT INTO material_brand (full_name) VALUES ('Proto-pasta');
INSERT INTO material_brand (full_name) VALUES ('ColorFabb');
INSERT INTO material_brand (full_name) VALUES ('Polymaker');


INSERT INTO printer (model, printer_brand_fk) VALUES ('Ender 3', (SELECT id FROM printer_brand WHERE full_name = 'Creality'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('Ender 3 Pro', (SELECT id FROM printer_brand WHERE full_name = 'Creality'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('i3 Mega', (SELECT id FROM printer_brand WHERE full_name = 'Anycubic'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('Photon', (SELECT id FROM printer_brand WHERE full_name = 'Anycubic'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('i3 MK3S', (SELECT id FROM printer_brand WHERE full_name = 'Prusa'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('S5', (SELECT id FROM printer_brand WHERE full_name = 'Ultimaker'));
INSERT INTO printer (model, printer_brand_fk) VALUES ('TAZ 6', (SELECT id FROM printer_brand WHERE full_name = 'LulzBot'));



INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Super Glue', 'Composite', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Eco Res', 'Resin', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Eco PLA', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Candy Apple Red', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Midnight Black', 'ABS', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Clear', 'PETG', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Wood Infused', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Translucent Green', 'PETG', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Metallic Silver', 'ABS', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Glow in the Dark', 'Composite', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Neon Pink', 'Resin', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Orange Crush', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Sunburst Yellow', 'PETG', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Deep Sea Blue', 'Composite', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Pure White', 'Resin', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Carbon Fiber Infused', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Electric Purple', 'PETG', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Transparent Blue', 'ABS', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Sakura Pink', 'Composite', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Luminous Green', 'Resin', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('True Grey', 'PLA', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Bronze Infused', 'PETG', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));
INSERT INTO material (description, mat_type, material_brand_fk) VALUES ('Translucent Yellow', 'ABS', (SELECT id FROM material_brand ORDER BY random() LIMIT 1));



DO
$do$
BEGIN
FOR i IN 1..100 LOOP
                INSERT INTO gcode (readme, file_pk)
                VALUES ('This is the readme',
                        (SELECT id FROM file ORDER BY random() LIMIT 1));
END LOOP;
END
$do$;



INSERT INTO print (material_fk, printer_fk, gcode_fk, nozzle_size_mm, bed_temp_celsius, successful, extruder_temp)
VALUES (
           (SELECT id FROM material ORDER BY random() LIMIT 1),
       (SELECT id FROM printer ORDER BY random() LIMIT 1),
       (SELECT id FROM gcode ORDER BY random() LIMIT 1),
       (SELECT ROUND((SELECT random()*(0.8-0.1)+0.1)::numeric, 1)),
       (SELECT random()*(80-40)+40),
       (SELECT RANDOM()::INT::BOOLEAN),
       (SELECT random()*(250-180)+180)
    );


DO
$do$
BEGIN
FOR i IN 1..500 LOOP
                INSERT INTO print (material_fk, printer_fk, gcode_fk, nozzle_size_mm, bed_temp_celsius, successful, extruder_temp)
                VALUES (
                           null,
                           null,
                           (SELECT id FROM gcode ORDER BY random() LIMIT 1),
                           (SELECT ROUND((SELECT random()*(0.8-0.1)+0.1)::numeric, 1)),
                           (SELECT random()*(80-40)+40),
                           (SELECT RANDOM()::INT::BOOLEAN),
                           (SELECT random()*(250-180)+180)
                       );
END LOOP;
END
$do$;


DO
$do$
    DECLARE
row_record file%ROWTYPE;
BEGIN
FOR row_record IN SELECT * FROM file LOOP BEGIN
                  insert into files_per_user(user_account_pk, roles_pk, files_pk) VALUES
                      (
                      (SELECT mail FROM user_account ORDER BY random() LIMIT 1),
                      'owner',
                      row_record.id
                      );
EXCEPTION WHEN unique_violation THEN
END;
END LOOP;
END
$do$;


DO
$do$
BEGIN
FOR i IN 1..100 LOOP BEGIN
            insert into files_per_user(user_account_pk, roles_pk, files_pk) VALUES
                (
                    (SELECT mail FROM user_account ORDER BY random() LIMIT 1),
                    (SELECT permission FROM file_permissions
                                       WHERE permission = 'read' or permission = 'delete'
                                        ORDER BY random() LIMIT 1),
                    (SELECT id FROM file ORDER BY random() LIMIT 1)
                );
EXCEPTION WHEN unique_violation THEN
END;
END LOOP;
END
$do$;
