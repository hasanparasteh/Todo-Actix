-- Your SQL goes here
CREATE TABLE todos (
    `id` INT AUTO_INCREMENT PRIMARY KEY,
    `title` VARCHAR(250) NOT NULL,
    `status` VARCHAR(64) NOT NULL,
    `created` INT(11) unsigned NOT NULL,
    `ended` INT(11) unsigned NOT NULL,
    `schedule` INT(11) unsigned NOT NULL,
    `is_schedule` TINYINT(1) NOT NULL
);