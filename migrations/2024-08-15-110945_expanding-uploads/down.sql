Alter table uploads
    rename to upload;
ALTER TABLE upload
    DROP COLUMN status;
ALTER TABLE upload
    DROP COLUMN message;