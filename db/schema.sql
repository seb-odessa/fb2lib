BEGIN TRANSACTION;
CREATE TABLE "status" (
	`statusId`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name`	TEXT NOT NULL,
	`comment`	TEXT
);
INSERT INTO `status` (statusId,name,comment) VALUES (1,'REGISTRED','Registred for processing. May change state any except UNDEFINED');
INSERT INTO `status` (statusId,name,comment) VALUES (2,'OPENNED','Openned by worker for processing');
INSERT INTO `status` (statusId,name,comment) VALUES (3,'PROCESSED','Processed by worker');
INSERT INTO `status` (statusId,name,comment) VALUES (4,'UNAVAILABLE','Resource (may be temporary) unavailable');
INSERT INTO `status` (statusId,name,comment) VALUES (5,'UNREGISTRED','Resource was unregistred');
INSERT INTO `status` (statusId,name,comment) VALUES (6,'FAILED','Worker was not able to process this resource');
CREATE TABLE "files" (
	`fileId`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	`containerId`	INTEGER,
	`path`	TEXT,
	`name`	TEXT NOT NULL,
	`md5`	TEXT,
	`statusId`	INTEGER NOT NULL DEFAULT 0,
	`changed`	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE "containers" (
	`containerId`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`path`	TEXT,
	`name`	TEXT NOT NULL,
	`md5`	TEXT,
	`statusId`	INTEGER NOT NULL DEFAULT 0,
	`changed`	TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX `files_name` ON `files` (`name` );
CREATE INDEX `files_md5` ON `files` (`md5` );
CREATE INDEX `containers_name` ON `containers` (`name` );
CREATE INDEX `containers_md5` ON `containers` (`md5` ASC);
COMMIT;
