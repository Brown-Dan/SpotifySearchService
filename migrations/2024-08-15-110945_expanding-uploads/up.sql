Alter table upload
    rename to uploads;
ALTER TABLE uploads
    ADD COLUMN status varchar(36) DEFAULT 'PENDING';
ALTER TABLE uploads
    ADD COLUMN message text;